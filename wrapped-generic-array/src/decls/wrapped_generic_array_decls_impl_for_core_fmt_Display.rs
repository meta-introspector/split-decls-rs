use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl core::fmt::Display for LengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("LengthError: Slice or iterator does not match GenericArray length")
    }
}
