// Generated macro for impl_479 (impl)
macro_rules! Depcrate_concurrency_init_onceimpl_479 {
() => {
// Module: crate::concurrency::init_once
// Provides: {"impl_479"}
// Dependencies: {}
impl InitOnceRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn status (& self) -> InitOnceStatus { self . 0 . borrow () . status () } pub fn begin (& self) { self . 0 . borrow_mut () . begin () ; } pub fn queue_is_empty (& self) -> bool { self . 0 . borrow () . waiters . is_empty () } }
};
}
