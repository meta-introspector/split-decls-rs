// Generated macro for is_cast_from_ty_alias (function)
macro_rules! Depcrate_casts_unnecessary_castis_cast_from_ty_alias {
() => {
// Module: crate::casts::unnecessary_cast
// Provides: {"is_cast_from_ty_alias"}
// Dependencies: {}
# [doc = " Finds whether an `Expr` returns a type alias."] # [doc = ""] # [doc = " TODO: Maybe we should move this to `clippy_utils` so others won't need to go down this dark,"] # [doc = " dark path reimplementing this (or something similar)."] fn is_cast_from_ty_alias < 'tcx > (cx : & LateContext < 'tcx > , expr : impl Visitable < 'tcx > , cast_from : Ty < 'tcx >) -> bool { for_each_expr_without_closures (expr , | expr | { if let ExprKind :: Path (qpath) = expr . kind { let res = cx . qpath_res (& qpath , expr . hir_id) ; if let Res :: Def (DefKind :: Fn , def_id) = res { let Some (snippet) = cx . tcx . def_span (def_id) . get_source_text (cx) else { return ControlFlow :: Continue (()) ; } ; if ! snippet . split ("->") . skip (1) . any (| s | snippet_eq_ty (s , cast_from) || s . split ("where") . any (| ty | snippet_eq_ty (ty , cast_from))) { return ControlFlow :: Break (()) ; } } else if let Res :: Local (hir_id) = res && let Node :: LetStmt (l) = cx . tcx . parent_hir_node (hir_id) { if let Some (e) = l . init && is_cast_from_ty_alias (cx , e , cast_from) { return ControlFlow :: Break :: < () > (()) ; } if let Some (ty) = l . ty && let TyKind :: Path (qpath) = ty . kind && is_ty_alias (& qpath) { return ControlFlow :: Break :: < () > (()) ; } } } ControlFlow :: Continue (()) }) . is_some () }
};
}
