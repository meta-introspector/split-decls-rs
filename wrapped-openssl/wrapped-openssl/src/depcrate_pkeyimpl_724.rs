// Generated macro for impl_724 (impl)
macro_rules! Depcrate_pkeyimpl_724 {
() => {
// Module: crate::pkey
// Provides: {"impl_724"}
// Dependencies: {}
# [cfg (not (boringssl))] impl < T > TryFrom < Dh < T > > for PKey < T > { type Error = ErrorStack ; fn try_from (dh : Dh < T >) -> Result < PKey < T > , ErrorStack > { PKey :: from_dh (dh) } }
};
}
