// Generated macro for impl_473 (impl)
macro_rules! Depcrate_concurrency_syncimpl_473 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_473"}
// Dependencies: {}
impl FutexRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn waiters (& self) -> usize { self . 0 . borrow () . waiters . len () } }
};
}
