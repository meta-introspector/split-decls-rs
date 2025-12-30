// Generated macro for impl_723 (impl)
macro_rules! Depcrate_pkeyimpl_723 {
() => {
// Module: crate::pkey
// Provides: {"impl_723"}
// Dependencies: {}
impl < T > TryFrom < PKey < T > > for Dsa < T > { type Error = ErrorStack ; fn try_from (pkey : PKey < T >) -> Result < Dsa < T > , ErrorStack > { pkey . dsa () } }
};
}
