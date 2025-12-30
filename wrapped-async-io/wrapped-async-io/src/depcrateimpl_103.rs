// Generated macro for impl_103 (impl)
macro_rules! Depcrateimpl_103 {
() => {
// Module: crate
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (windows)] impl < T : Into < OwnedSocket > > TryFrom < Async < T > > for OwnedSocket { type Error = io :: Error ; fn try_from (value : Async < T >) -> Result < Self , Self :: Error > { value . into_inner () . map (Into :: into) } }
};
}
