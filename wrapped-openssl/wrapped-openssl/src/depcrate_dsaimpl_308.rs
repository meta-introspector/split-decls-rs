// Generated macro for impl_308 (impl)
macro_rules! Depcrate_dsaimpl_308 {
() => {
// Module: crate::dsa
// Provides: {"impl_308"}
// Dependencies: {}
impl < T > ToOwned for DsaRef < T > { type Owned = Dsa < T > ; fn to_owned (& self) -> Dsa < T > { unsafe { ffi :: DSA_up_ref (self . as_ptr ()) ; Dsa :: from_ptr (self . as_ptr ()) } } }
};
}
