use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Platform::Name(ref n) => n.fmt(f),
            Platform::Cfg(ref e) => write!(f, "cfg({})", e),
        }
    }
}
