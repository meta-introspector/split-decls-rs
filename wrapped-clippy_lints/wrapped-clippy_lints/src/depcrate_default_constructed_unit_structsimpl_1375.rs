// Generated macro for impl_1375 (impl)
macro_rules! Depcrate_default_constructed_unit_structsimpl_1375 {
() => {
// Module: crate::default_constructed_unit_structs
// Provides: {"impl_1375"}
// Dependencies: {}
impl LateLintPass < '_ > for DefaultConstructedUnitStructs { fn check_expr < 'tcx > (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if let ExprKind :: Call (fn_expr , & []) = expr . kind && let ExprKind :: Path (ref qpath @ hir :: QPath :: TypeRelative (base , _)) = fn_expr . kind && ! is_alias (* base) && let Res :: Def (_ , def_id) = cx . qpath_res (qpath , fn_expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: default_fn , def_id) && let ty :: Adt (def , ..) = cx . typeck_results () . expr_ty (expr) . kind () && def . is_struct () && let var @ ty :: VariantDef { ctor : Some ((hir :: def :: CtorKind :: Const , _)) , .. } = def . non_enum_variant () && ! var . is_field_list_non_exhaustive () && ! expr . span . from_expansion () && ! qpath . span () . from_expansion () && ! base . is_suggestable_infer_ty () { let mut removals = vec ! [(expr . span . with_lo (qpath . qself_span () . hi ()) , String :: new ())] ; if expr . span . with_source_text (cx , | s | s . starts_with ('<')) == Some (true) { removals . push ((expr . span . with_hi (qpath . qself_span () . lo ()) , String :: new ())) ; } span_lint_and_then (cx , DEFAULT_CONSTRUCTED_UNIT_STRUCTS , expr . span , "use of `default` to create a unit struct" , | diag | { diag . multipart_suggestion ("remove this call to `default`" , removals , Applicability :: MachineApplicable ,) ; } ,) ; } } }
};
}
