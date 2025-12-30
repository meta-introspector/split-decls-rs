// Generated macro for impl_720 (impl)
macro_rules! Depcrate_pkeyimpl_720 {
() => {
// Module: crate::pkey
// Provides: {"impl_720"}
// Dependencies: {}
impl < T > TryFrom < Rsa < T > > for PKey < T > { type Error = ErrorStack ; fn try_from (rsa : Rsa < T >) -> Result < PKey < T > , ErrorStack > { PKey :: from_rsa (rsa) } }
};
}
