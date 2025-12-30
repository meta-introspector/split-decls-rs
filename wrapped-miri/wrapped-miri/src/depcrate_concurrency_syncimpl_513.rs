// Generated macro for impl_513 (impl)
macro_rules! Depcrate_concurrency_syncimpl_513 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_513"}
// Dependencies: {}
impl CondvarRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn queue_is_empty (& self) -> bool { self . 0 . borrow () . waiters . is_empty () } }
};
}
