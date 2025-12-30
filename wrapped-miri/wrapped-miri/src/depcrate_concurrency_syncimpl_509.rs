// Generated macro for impl_509 (impl)
macro_rules! Depcrate_concurrency_syncimpl_509 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_509"}
// Dependencies: {}
impl RwLockRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn is_locked (& self) -> bool { self . 0 . borrow () . is_locked () } pub fn is_write_locked (& self) -> bool { self . 0 . borrow () . is_write_locked () } pub fn queue_is_empty (& self) -> bool { let inner = self . 0 . borrow () ; inner . reader_queue . is_empty () && inner . writer_queue . is_empty () } }
};
}
