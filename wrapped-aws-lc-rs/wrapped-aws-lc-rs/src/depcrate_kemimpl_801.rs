// Generated macro for impl_801 (impl)
macro_rules! Depcrate_kemimpl_801 {
() => {
// Module: crate::kem
// Provides: {"impl_801"}
// Dependencies: {}
impl Drop for Ciphertext < '_ > { fn drop (& mut self) { if let Cow :: Owned (ref mut v) = self . 0 { v . zeroize () ; } } }
};
}
