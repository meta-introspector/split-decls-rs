// Generated macro for walk_fn_ret_ty (function)
macro_rules! Depcrate_intravisitwalk_fn_ret_ty {
() => {
// Module: crate::intravisit
// Provides: {"walk_fn_ret_ty"}
// Dependencies: {}
pub fn walk_fn_ret_ty < 'v , V : Visitor < 'v > > (visitor : & mut V , ret_ty : & 'v FnRetTy < 'v >) -> V :: Result { if let FnRetTy :: Return (output_ty) = * ret_ty { try_visit ! (visitor . visit_ty_unambig (output_ty)) ; } V :: Result :: output () }
};
}
