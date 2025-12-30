// Generated macro for impl_3080 (impl)
macro_rules! Depcrate_init_numbered_fieldsimpl_3080 {
() => {
// Module: crate::init_numbered_fields
// Provides: {"impl_3080"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NumberedFields { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if let ExprKind :: Struct (path , fields @ [field , ..] , StructTailExpr :: None) = e . kind && field . ident . as_str () . as_bytes () . first () . is_some_and (u8 :: is_ascii_digit) && ! matches ! (cx . qpath_res (path , e . hir_id) , Res :: Def (DefKind :: TyAlias | DefKind :: AssocTy , _)) && ! e . span . from_expansion () && let mut has_side_effects = false && let Ok (mut expr_spans) = fields . iter () . map (| f | { has_side_effects |= f . expr . can_have_side_effects () ; f . ident . as_str () . parse :: < usize > () . map (| x | (x , f . expr . span)) }) . collect :: < Result < Vec < _ > , _ > > () && (! has_side_effects || expr_spans . is_sorted_by_key (| & (idx , _) | idx)) { span_lint_and_then (cx , INIT_NUMBERED_FIELDS , e . span , "used a field initializer for a tuple struct" , | diag | { if ! has_side_effects { expr_spans . sort_by_key (| & (idx , _) | idx) ; } let mut app = Applicability :: MachineApplicable ; diag . span_suggestion (e . span , "use tuple initialization" , format ! ("{}({})" , snippet_with_applicability (cx , path . span () , ".." , & mut app) , expr_spans . into_iter () . map (| (_ , span) | snippet_with_context (cx , span , SyntaxContext :: root () , ".." , & mut app) . 0) . intersperse (Cow :: Borrowed (", ")) . collect ::< String > ()) , app ,) ; } ,) ; } } }
};
}
