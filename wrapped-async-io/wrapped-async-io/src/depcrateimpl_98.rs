// Generated macro for impl_98 (impl)
macro_rules! Depcrateimpl_98 {
() => {
// Module: crate
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (unix)] impl < T : Into < OwnedFd > > TryFrom < Async < T > > for OwnedFd { type Error = io :: Error ; fn try_from (value : Async < T >) -> Result < Self , Self :: Error > { value . into_inner () . map (Into :: into) } }
};
}
