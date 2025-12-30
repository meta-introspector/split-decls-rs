// Generated macro for get_data_type (function)
macro_rules! Depcrate_methods_err_expectget_data_type {
() => {
// Module: crate::methods::err_expect
// Provides: {"get_data_type"}
// Dependencies: {}
# [doc = " Given a `Result<T, E>` type, return its data (`T`)."] fn get_data_type < 'a > (cx : & LateContext < '_ > , ty : Ty < 'a >) -> Option < Ty < 'a > > { match ty . kind () { ty :: Adt (adt , args) if cx . tcx . is_diagnostic_item (sym :: Result , adt . did ()) => args . types () . next () , _ => None , } }
};
}
