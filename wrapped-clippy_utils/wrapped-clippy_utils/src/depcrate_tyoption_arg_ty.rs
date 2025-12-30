// Generated macro for option_arg_ty (function)
macro_rules! Depcrate_tyoption_arg_ty {
() => {
// Module: crate::ty
// Provides: {"option_arg_ty"}
// Dependencies: {}
# [doc = " Check if `ty` is an `Option` and return its argument type if it is."] pub fn option_arg_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { match * ty . kind () { ty :: Adt (adt , args) if let [arg] = & * * args && let Some (arg) = arg . as_type () && adt . is_diag_item (cx , sym :: Option) => { Some (arg) } , _ => None , } }
};
}
