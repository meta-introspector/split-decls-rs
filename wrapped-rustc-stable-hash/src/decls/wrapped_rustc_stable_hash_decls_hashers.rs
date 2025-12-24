use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Hashers collection
pub mod hashers {
    #[doc(inline)]
    pub use super::sip128::{SipHasher128, SipHasher128Hash};
    /// Stable 128-bits Sip Hasher
    ///
    /// [`StableHasher`] version of [`SipHasher128`].
    ///
    /// [`StableHasher`]: super::StableHasher
    pub type StableSipHasher128 = super::StableHasher<SipHasher128>;
}
