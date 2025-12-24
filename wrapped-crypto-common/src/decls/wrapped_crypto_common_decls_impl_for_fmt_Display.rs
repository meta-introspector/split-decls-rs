use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for WeakKeyError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("WeakKey")
    }
}
