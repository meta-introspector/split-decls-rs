// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "ptr"))] impl AtomicUsize { const fn new (v : usize) -> AtomicUsize { AtomicUsize { v : Cell :: new (v) } } fn load (& self , _order : Ordering) -> usize { self . v . get () } fn store (& self , val : usize , _order : Ordering) { self . v . set (val) } }
};
}
