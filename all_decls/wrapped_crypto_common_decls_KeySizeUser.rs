use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which use key for initialization.
///
/// Generally it's used indirectly via [`KeyInit`] or [`KeyIvInit`].
pub trait KeySizeUser {
    /// Key size in bytes.
    type KeySize: ArraySize;
    /// Return key size in bytes.
    #[inline(always)]
    fn key_size() -> usize {
        Self::KeySize::USIZE
    }
}
