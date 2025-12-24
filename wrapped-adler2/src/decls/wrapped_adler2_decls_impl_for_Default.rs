use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Adler32 {
    #[inline]
    fn default() -> Self {
        Adler32 { a: 1, b: 0 }
    }
}
