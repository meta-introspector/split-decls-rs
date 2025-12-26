use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Buffer {
    #[inline]
    fn default() -> Buffer {
        Buffer::new()
    }
}
