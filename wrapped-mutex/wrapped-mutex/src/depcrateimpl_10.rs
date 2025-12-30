// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl SpinBarrier { pub const fn new (n : usize) -> Self { Self { num_threads : AtomicUsize :: new (n) , } } pub fn wait (& self) { self . num_threads . fetch_sub (1 , Ordering :: Relaxed) ; while self . num_threads . load (Ordering :: Relaxed) != 0 { hint :: spin_loop () ; } } }
};
}
