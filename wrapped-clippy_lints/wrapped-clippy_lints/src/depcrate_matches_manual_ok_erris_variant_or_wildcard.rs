// Generated macro for is_variant_or_wildcard (function)
macro_rules! Depcrate_matches_manual_ok_erris_variant_or_wildcard {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"is_variant_or_wildcard"}
// Dependencies: {}
# [doc = " Check that `pat` applied to a `Result` only matches `Ok(_)`, `Err(_)`, not a subset or a"] # [doc = " superset of it. If `can_be_wild` is `true`, wildcards are also accepted. In the case of"] # [doc = " a non-wildcard, `must_match_err` indicates whether the `Err` or the `Ok` variant should be"] # [doc = " accepted."] fn is_variant_or_wildcard (cx : & LateContext < '_ > , pat : & Pat < '_ > , can_be_wild : bool , must_match_err : bool) -> bool { match pat . kind { PatKind :: Wild | PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (_) , .. }) | PatKind :: Binding (_ , _ , _ , None) if can_be_wild => { true } , PatKind :: TupleStruct (qpath , ..) => { cx . qpath_res (& qpath , pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , ResultErr) == must_match_err } , PatKind :: Binding (_ , _ , _ , Some (pat)) | PatKind :: Ref (pat , _ , _) => { is_variant_or_wildcard (cx , pat , can_be_wild , must_match_err) } , _ => false , } }
};
}
