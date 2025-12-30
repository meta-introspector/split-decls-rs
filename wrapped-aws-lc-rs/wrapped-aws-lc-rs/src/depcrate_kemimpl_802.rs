// Generated macro for impl_802 (impl)
macro_rules! Depcrate_kemimpl_802 {
() => {
// Module: crate::kem
// Provides: {"impl_802"}
// Dependencies: {}
impl AsRef < [u8] > for Ciphertext < '_ > { fn as_ref (& self) -> & [u8] { match self . 0 { Cow :: Borrowed (v) => v , Cow :: Owned (ref v) => v . as_ref () , } } }
};
}
