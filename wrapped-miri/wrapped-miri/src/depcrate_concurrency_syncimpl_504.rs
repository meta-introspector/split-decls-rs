// Generated macro for impl_504 (impl)
macro_rules! Depcrate_concurrency_syncimpl_504 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_504"}
// Dependencies: {}
impl MutexRef { pub fn new () -> Self { Self (Default :: default ()) } # [doc = " Get the id of the thread that currently owns this lock, or `None` if it is not locked."] pub fn owner (& self) -> Option < ThreadId > { self . 0 . borrow () . owner } pub fn queue_is_empty (& self) -> bool { self . 0 . borrow () . queue . is_empty () } }
};
}
