// Generated macro for hashers (module)
macro_rules! Depcratehashers {
() => {
// Module: crate
// Provides: {"hashers"}
// Dependencies: {}
# [doc = " Hashers collection"] pub mod hashers { # [doc (inline)] pub use super :: sip128 :: { SipHasher128 , SipHasher128Hash } ; # [doc = " Stable 128-bits Sip Hasher"] # [doc = ""] # [doc = " [`StableHasher`] version of [`SipHasher128`]."] # [doc = ""] # [doc = " [`StableHasher`]: super::StableHasher"] pub type StableSipHasher128 = super :: StableHasher < SipHasher128 > ; }
};
}
