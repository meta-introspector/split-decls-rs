// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl AtomicUsize { const fn new (v : usize) -> AtomicUsize { AtomicUsize { v : Cell :: new (v) } } fn load (& self , _order : Ordering) -> usize { self . v . get () } fn store (& self , val : usize , _order : Ordering) { self . v . set (val) } }
};
}
