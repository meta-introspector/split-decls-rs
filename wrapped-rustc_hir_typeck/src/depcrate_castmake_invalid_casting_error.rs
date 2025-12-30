// Generated macro for make_invalid_casting_error (function)
macro_rules! Depcrate_castmake_invalid_casting_error {
() => {
// Module: crate::cast
// Provides: {"make_invalid_casting_error"}
// Dependencies: {}
fn make_invalid_casting_error < 'a , 'tcx > (span : Span , expr_ty : Ty < 'tcx > , cast_ty : Ty < 'tcx > , fcx : & FnCtxt < 'a , 'tcx > ,) -> Diag < 'a > { type_error_struct ! (fcx . dcx () , span , expr_ty , E0606 , "casting `{}` as `{}` is invalid" , fcx . ty_to_string (expr_ty) , fcx . ty_to_string (cast_ty)) }
};
}
