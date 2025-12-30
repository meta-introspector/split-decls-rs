// Generated macro for impl_721 (impl)
macro_rules! Depcrate_pkeyimpl_721 {
() => {
// Module: crate::pkey
// Provides: {"impl_721"}
// Dependencies: {}
impl < T > TryFrom < PKey < T > > for Rsa < T > { type Error = ErrorStack ; fn try_from (pkey : PKey < T >) -> Result < Rsa < T > , ErrorStack > { pkey . rsa () } }
};
}
