// Generated macro for impl_517 (impl)
macro_rules! Depcrate_concurrency_syncimpl_517 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_517"}
// Dependencies: {}
impl FutexRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn waiters (& self) -> usize { self . 0 . borrow () . waiters . len () } }
};
}
