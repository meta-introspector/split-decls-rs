// Generated macro for something_with_a_dtor (function)
macro_rules! Depcratesomething_with_a_dtor {
() => {
// Module: crate
// Provides: {"something_with_a_dtor"}
// Dependencies: {}
fn something_with_a_dtor (f : & dyn Fn ()) { struct A < 'a > (& 'a (dyn Fn () + 'a)) ; impl Drop for A < '_ > { fn drop (& mut self) { (self . 0) () ; } } let _a = A (f) ; f () ; }
};
}
