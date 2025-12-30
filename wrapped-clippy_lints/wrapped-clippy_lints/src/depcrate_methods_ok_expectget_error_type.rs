// Generated macro for get_error_type (function)
macro_rules! Depcrate_methods_ok_expectget_error_type {
() => {
// Module: crate::methods::ok_expect
// Provides: {"get_error_type"}
// Dependencies: {}
# [doc = " Given a `Result<T, E>` type, return its error type (`E`)."] fn get_error_type < 'a > (cx : & LateContext < '_ > , ty : Ty < 'a >) -> Option < Ty < 'a > > { match ty . kind () { ty :: Adt (adt , args) if cx . tcx . is_diagnostic_item (sym :: Result , adt . did ()) => args . types () . nth (1) , _ => None , } }
};
}
