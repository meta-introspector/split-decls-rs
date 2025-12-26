use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which use initialization vector (nonce) for initialization.
///
/// Generally it's used indirectly via [`KeyIvInit`] or [`InnerIvInit`].
pub trait IvSizeUser {
    /// Initialization vector size in bytes.
    type IvSize: ArraySize;
    /// Return IV size in bytes.
    #[inline(always)]
    fn iv_size() -> usize {
        Self::IvSize::USIZE
    }
}
