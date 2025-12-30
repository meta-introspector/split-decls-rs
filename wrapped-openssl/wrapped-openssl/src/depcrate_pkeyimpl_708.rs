// Generated macro for impl_708 (impl)
macro_rules! Depcrate_pkeyimpl_708 {
() => {
// Module: crate::pkey
// Provides: {"impl_708"}
// Dependencies: {}
impl < T > ToOwned for PKeyRef < T > { type Owned = PKey < T > ; fn to_owned (& self) -> PKey < T > { unsafe { EVP_PKEY_up_ref (self . as_ptr ()) ; PKey :: from_ptr (self . as_ptr ()) } } }
};
}
