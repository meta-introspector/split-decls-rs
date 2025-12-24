use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl error::Error for FromOsStrError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
