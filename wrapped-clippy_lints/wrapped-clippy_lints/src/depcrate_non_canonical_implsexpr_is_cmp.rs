// Generated macro for expr_is_cmp (function)
macro_rules! Depcrate_non_canonical_implsexpr_is_cmp {
() => {
// Module: crate::non_canonical_impls
// Provides: {"expr_is_cmp"}
// Dependencies: {}
# [doc = " Return true if `expr_kind` is a `cmp` call."] fn expr_is_cmp < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , impl_item : & ImplItem < '_ > , needs_fully_qualified : & mut bool ,) -> bool { let typeck = cx . tcx . typeck (impl_item . owner_id . def_id) ; match expr . kind { ExprKind :: Call (Expr { kind : ExprKind :: Path (some_path) , hir_id : some_hir_id , .. } , [cmp_expr] ,) => { typeck . qpath_res (some_path , * some_hir_id) . ctor_parent (cx) . is_lang_item (cx , LangItem :: OptionSome) && self_cmp_call (cx , typeck , cmp_expr , needs_fully_qualified) } , ExprKind :: MethodCall (_ , recv , [] , _) => { typeck . type_dependent_def (expr . hir_id) . assoc_parent (cx) . is_diag_item (cx , sym :: Into) && self_cmp_call (cx , typeck , recv , needs_fully_qualified) } , _ => false , } }
};
}
