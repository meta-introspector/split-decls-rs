use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(clippy::non_canonical_clone_impl)]
impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        Buffer::new()
    }
}
