// Generated macro for impl_719 (impl)
macro_rules! Depcrate_pkeyimpl_719 {
() => {
// Module: crate::pkey
// Provides: {"impl_719"}
// Dependencies: {}
impl < T > TryFrom < PKey < T > > for EcKey < T > { type Error = ErrorStack ; fn try_from (pkey : PKey < T >) -> Result < EcKey < T > , ErrorStack > { pkey . ec_key () } }
};
}
