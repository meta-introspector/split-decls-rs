// Generated macro for is_inside_always_const_context (function)
macro_rules! Depcrateis_inside_always_const_context {
() => {
// Module: crate
// Provides: {"is_inside_always_const_context"}
// Dependencies: {}
# [doc = " Returns `true` if the given `HirId` is inside an always constant context."] # [doc = ""] # [doc = " This context includes:"] # [doc = "  * const/static items"] # [doc = "  * const blocks (or inline consts)"] # [doc = "  * associated constants"] pub fn is_inside_always_const_context (tcx : TyCtxt < '_ > , hir_id : HirId) -> bool { use rustc_hir :: ConstContext :: { Const , ConstFn , Static } ; let Some (ctx) = tcx . hir_body_const_context (tcx . hir_enclosing_body_owner (hir_id)) else { return false ; } ; match ctx { ConstFn => false , Static (_) | Const { inline : _ } => true , } }
};
}
