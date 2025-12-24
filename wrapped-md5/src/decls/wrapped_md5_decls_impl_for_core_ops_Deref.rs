use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl core::ops::Deref for Digest {
    type Target = [u8; 16];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
