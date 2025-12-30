// Generated macro for impl_725 (impl)
macro_rules! Depcrate_pkeyimpl_725 {
() => {
// Module: crate::pkey
// Provides: {"impl_725"}
// Dependencies: {}
impl < T > TryFrom < PKey < T > > for Dh < T > { type Error = ErrorStack ; fn try_from (pkey : PKey < T >) -> Result < Dh < T > , ErrorStack > { pkey . dh () } }
};
}
