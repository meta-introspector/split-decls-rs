// Generated macro for impl_155 (impl)
macro_rules! Depcrate_arcimpl_155 {
() => {
// Module: crate::arc
// Provides: {"impl_155"}
// Dependencies: {}
impl < T : ? Sized > Deref for Arc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { & self . inner () . data } }
};
}
