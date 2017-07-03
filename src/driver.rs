use document::{ModPath};
use convert::NewDocTemp_;
use store::{self, StoreLocation};
use errors::*;

mod errors {
    error_chain! {
        errors {
            NoDocumentationFound {
                description("No documentation could be found.")
            }
        }
    }
}

pub struct Driver {}

impl Driver {
    pub fn new() -> Driver {
        Driver { }
    }

    pub fn get_doc(location: &StoreLocation) -> Result<NewDocTemp_> {
        let path = location.to_filepath();
        store::deserialize_object(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
