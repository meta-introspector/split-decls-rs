use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Debug for NothingPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?")
    }
}
