// Generated macro for impl_718 (impl)
macro_rules! Depcrate_pkeyimpl_718 {
() => {
// Module: crate::pkey
// Provides: {"impl_718"}
// Dependencies: {}
impl < T > TryFrom < EcKey < T > > for PKey < T > { type Error = ErrorStack ; fn try_from (ec_key : EcKey < T >) -> Result < PKey < T > , ErrorStack > { PKey :: from_ec_key (ec_key) } }
};
}
