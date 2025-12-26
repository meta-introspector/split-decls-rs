use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsRef<[u8]> for ObjectIdentifierRef {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
