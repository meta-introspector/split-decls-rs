// Generated macro for impl_23 (impl)
macro_rules! Depcrate_almost_standard_lint_formulationimpl_23 {
() => {
// Module: crate::almost_standard_lint_formulation
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AlmostStandardFormulation { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { let mut check_next = false ; if let ItemKind :: Static (Mutability :: Not , _ , ty , _) = item . kind { let lines = cx . tcx . hir_attrs (item . hir_id ()) . iter () . filter_map (| attr | Attribute :: doc_str (attr) . map (| sym | (sym , attr))) ; if is_lint_ref_type (cx , ty) { for (line , attr) in lines { let cur_line = line . as_str () . trim () ; if check_next && ! cur_line . is_empty () { for formulation in & self . standard_formulations { let starts_with_correct_formulation = cur_line . starts_with (formulation . correction) ; if ! starts_with_correct_formulation && formulation . wrong_pattern . is_match (cur_line) { if let Some (ident) = attr . ident () { span_lint_and_help (cx , ALMOST_STANDARD_LINT_FORMULATION , ident . span , "non-standard lint formulation" , None , format ! ("consider using `{}`" , formulation . correction) ,) ; } return ; } } return ; } else if cur_line . contains ("What it does") { check_next = true ; } else if cur_line . contains ("Why is this bad") { return ; } } } } } }
};
}
