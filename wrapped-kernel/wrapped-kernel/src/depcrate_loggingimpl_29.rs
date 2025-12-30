// Generated macro for impl_29 (impl)
macro_rules! Depcrate_loggingimpl_29 {
() => {
// Module: crate::logging
// Provides: {"impl_29"}
// Dependencies: {}
impl KernelLogger { pub const fn new () -> Self { Self { time : AtomicBool :: new (false) , } } pub fn time (& self) -> bool { self . time . load (Ordering :: Relaxed) } pub fn set_time (& self , time : bool) { self . time . store (time , Ordering :: Relaxed) ; } }
};
}
