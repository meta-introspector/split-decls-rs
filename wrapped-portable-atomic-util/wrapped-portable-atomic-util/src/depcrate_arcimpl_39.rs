// Generated macro for impl_39 (impl)
macro_rules! Depcrate_arcimpl_39 {
() => {
// Module: crate::arc
// Provides: {"impl_39"}
// Dependencies: {}
impl < T : ? Sized > Deref for Arc < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . inner () . data } }
};
}
