// Generated macro for impl_722 (impl)
macro_rules! Depcrate_pkeyimpl_722 {
() => {
// Module: crate::pkey
// Provides: {"impl_722"}
// Dependencies: {}
impl < T > TryFrom < Dsa < T > > for PKey < T > { type Error = ErrorStack ; fn try_from (dsa : Dsa < T >) -> Result < PKey < T > , ErrorStack > { PKey :: from_dsa (dsa) } }
};
}
