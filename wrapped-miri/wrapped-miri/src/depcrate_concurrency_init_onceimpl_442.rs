// Generated macro for impl_442 (impl)
macro_rules! Depcrate_concurrency_init_onceimpl_442 {
() => {
// Module: crate::concurrency::init_once
// Provides: {"impl_442"}
// Dependencies: {}
impl InitOnceRef { pub fn new () -> Self { Self (Default :: default ()) } pub fn status (& self) -> InitOnceStatus { self . 0 . borrow () . status () } pub fn begin (& self) { self . 0 . borrow_mut () . begin () ; } }
};
}
