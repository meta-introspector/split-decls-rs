// Generated macro for check_impl_item (function)
macro_rules! Depcrate_methods_new_ret_no_selfcheck_impl_item {
() => {
// Module: crate::methods::new_ret_no_self
// Provides: {"check_impl_item"}
// Dependencies: {}
pub (super) fn check_impl_item < 'tcx > (cx : & LateContext < 'tcx > , impl_item : & 'tcx ImplItem < '_ > , self_ty : Ty < 'tcx > , implements_trait : bool ,) { if ! implements_trait && impl_item . ident . name == sym :: new && let ret_ty = return_ty (cx , impl_item . owner_id) && ret_ty != self_ty && ! contains_ty_adt_constructor_opaque (cx , ret_ty , self_ty) { span_lint (cx , NEW_RET_NO_SELF , impl_item . span , "methods called `new` usually return `Self`" ,) ; } }
};
}
