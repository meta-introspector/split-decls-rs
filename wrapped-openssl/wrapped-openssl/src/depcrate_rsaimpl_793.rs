// Generated macro for impl_793 (impl)
macro_rules! Depcrate_rsaimpl_793 {
() => {
// Module: crate::rsa
// Provides: {"impl_793"}
// Dependencies: {}
impl < T > ToOwned for RsaRef < T > { type Owned = Rsa < T > ; fn to_owned (& self) -> Rsa < T > { unsafe { ffi :: RSA_up_ref (self . as_ptr ()) ; Rsa :: from_ptr (self . as_ptr ()) } } }
};
}
