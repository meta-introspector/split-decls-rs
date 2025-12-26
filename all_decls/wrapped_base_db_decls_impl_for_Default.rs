use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Nonce {
    #[inline]
    fn default() -> Self {
        Nonce::new()
    }
}
