use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl core::ops::DerefMut for Digest {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
