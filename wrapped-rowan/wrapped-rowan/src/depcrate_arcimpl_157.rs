// Generated macro for impl_157 (impl)
macro_rules! Depcrate_arcimpl_157 {
() => {
// Module: crate::arc
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : ? Sized > Drop for Arc < T > { # [inline] fn drop (& mut self) { if self . inner () . count . fetch_sub (1 , Release) != 1 { return ; } self . inner () . count . load (Acquire) ; unsafe { self . drop_slow () ; } } }
};
}
