// Generated macro for impl_353 (impl)
macro_rules! Depcrate_ecimpl_353 {
() => {
// Module: crate::ec
// Provides: {"impl_353"}
// Dependencies: {}
impl < T > ToOwned for EcKeyRef < T > { type Owned = EcKey < T > ; fn to_owned (& self) -> EcKey < T > { unsafe { let r = ffi :: EC_KEY_up_ref (self . as_ptr ()) ; assert ! (r == 1) ; EcKey :: from_ptr (self . as_ptr ()) } } }
};
}
