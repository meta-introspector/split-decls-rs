// Generated macro for impl_465 (impl)
macro_rules! Depcrate_concurrency_syncimpl_465 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_465"}
// Dependencies: {}
impl RwLockRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn is_locked (& self) -> bool { self . 0 . borrow () . is_locked () } pub fn is_write_locked (& self) -> bool { self . 0 . borrow () . is_write_locked () } }
};
}
