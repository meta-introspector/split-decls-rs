// Generated macro for check_trait_item (function)
macro_rules! Depcrate_methods_new_ret_no_selfcheck_trait_item {
() => {
// Module: crate::methods::new_ret_no_self
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item < 'tcx > (cx : & LateContext < 'tcx > , trait_item : & 'tcx TraitItem < 'tcx >) { if trait_item . ident . name == sym :: new && let ret_ty = return_ty (cx , trait_item . owner_id) && let self_ty = ty :: TraitRef :: identity (cx . tcx , trait_item . owner_id . to_def_id ()) . self_ty () && ! ret_ty . contains (self_ty) { span_lint (cx , NEW_RET_NO_SELF , trait_item . span , "methods called `new` usually return `Self`" ,) ; } }
};
}
