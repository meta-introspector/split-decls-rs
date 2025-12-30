// Generated macro for impl_121 (impl)
macro_rules! Depcrate_builtinimpl_121 {
() => {
// Module: crate::builtin
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for TypeAliasBounds { fn check_item (& mut self , cx : & LateContext < '_ > , item : & hir :: Item < '_ >) { let hir :: ItemKind :: TyAlias (_ , generics , hir_ty) = item . kind else { return } ; if generics . predicates . is_empty () { return ; } if cx . tcx . type_alias_is_lazy (item . owner_id) { return ; } let ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () ; if ty . has_type_flags (ty :: TypeFlags :: HAS_CT_PROJECTION) && cx . tcx . features () . generic_const_exprs () { return ; } let mut where_spans = Vec :: new () ; let mut inline_spans = Vec :: new () ; let mut inline_sugg = Vec :: new () ; for p in generics . predicates { let span = p . span ; if p . kind . in_where_clause () { where_spans . push (span) ; } else { for b in p . kind . bounds () { inline_spans . push (b . span ()) ; } inline_sugg . push ((span , String :: new ())) ; } } let mut ty = Some (hir_ty) ; let enable_feat_help = cx . tcx . sess . is_nightly_build () ; if let [.. , label_sp] = * where_spans { cx . emit_span_lint (TYPE_ALIAS_BOUNDS , where_spans , BuiltinTypeAliasBounds { in_where_clause : true , label : label_sp , enable_feat_help , suggestions : vec ! [(generics . where_clause_span , String :: new ())] , preds : generics . predicates , ty : ty . take () , } ,) ; } if let [.. , label_sp] = * inline_spans { cx . emit_span_lint (TYPE_ALIAS_BOUNDS , inline_spans , BuiltinTypeAliasBounds { in_where_clause : false , label : label_sp , enable_feat_help , suggestions : inline_sugg , preds : generics . predicates , ty , } ,) ; } } }
};
}
