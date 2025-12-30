// Generated macro for lower_const_arg_for_rustdoc (function)
macro_rules! Depcratelower_const_arg_for_rustdoc {
() => {
// Module: crate
// Provides: {"lower_const_arg_for_rustdoc"}
// Dependencies: {}
# [doc = " This is for rustdoc."] pub fn lower_const_arg_for_rustdoc < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ct : & hir :: ConstArg < 'tcx > , feed : FeedConstTy < '_ , 'tcx > ,) -> Const < 'tcx > { let env_def_id = tcx . hir_get_parent_item (hir_ct . hir_id) ; collect :: ItemCtxt :: new (tcx , env_def_id . def_id) . lowerer () . lower_const_arg (hir_ct , feed) }
};
}
