// Generated macro for KeyType (trait)
macro_rules! Depcrate_hkdfKeyType {
() => {
// Module: crate::hkdf
// Provides: {"KeyType"}
// Dependencies: {}
# [doc = " The length of the OKM (Output Keying Material) for a `Prk::expand()` call."] # [allow (clippy :: len_without_is_empty)] pub trait KeyType { # [doc = " The length that `Prk::expand()` should expand its input to."] fn len (& self) -> usize ; }
};
}
