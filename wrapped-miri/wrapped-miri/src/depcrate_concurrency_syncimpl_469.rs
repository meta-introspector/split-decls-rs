// Generated macro for impl_469 (impl)
macro_rules! Depcrate_concurrency_syncimpl_469 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_469"}
// Dependencies: {}
impl CondvarRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn is_awaited (& self) -> bool { ! self . 0 . borrow () . waiters . is_empty () } }
};
}
