// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
# [cfg (windows)] impl < T : AsSocket + From < OwnedSocket > > TryFrom < OwnedSocket > for Async < T > { type Error = io :: Error ; fn try_from (value : OwnedSocket) -> Result < Self , Self :: Error > { Async :: new (value . into ()) } }
};
}
