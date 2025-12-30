// Generated macro for impl_4211 (impl)
macro_rules! Depcrate_manual_ignore_case_cmpimpl_4211 {
() => {
// Module: crate::manual_ignore_case_cmp
// Provides: {"impl_4211"}
// Dependencies: {}
impl LateLintPass < '_ > for ManualIgnoreCaseCmp { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let Binary (op , left , right) = & expr . kind && (op . node == BinOpKind :: Eq || op . node == BinOpKind :: Ne) && let Some ((left_span , left_val)) = get_ascii_type (cx , left . kind) && let Some ((right_span , right_val)) = get_ascii_type (cx , right . kind) && match (& left_val , & right_val) { (ToAscii (l_lower , ..) , ToAscii (r_lower , ..)) if l_lower == r_lower => true , (ToAscii (..) , Literal (..)) | (Literal (..) , ToAscii (..)) => true , _ => false , } { let deref = match right_val { ToAscii (_ , ty) if needs_ref_to_cmp (cx , ty) => "&" , ToAscii (..) => "" , Literal (ty) => { if let LitKind :: Char (_) | LitKind :: Byte (_) = ty { "&" } else { "" } } , } ; let neg = if op . node == BinOpKind :: Ne { "!" } else { "" } ; span_lint_and_then (cx , MANUAL_IGNORE_CASE_CMP , expr . span , "manual case-insensitive ASCII comparison" , | diag | { let mut app = Applicability :: MachineApplicable ; diag . span_suggestion_verbose (expr . span , "consider using `.eq_ignore_ascii_case()` instead" , format ! ("{neg}{}.eq_ignore_ascii_case({deref}{})" , snippet_with_applicability (cx , left_span , "_" , & mut app) , snippet_with_applicability (cx , right_span , "_" , & mut app)) , app ,) ; } ,) ; } } }
};
}
