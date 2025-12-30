// Generated macro for impl_10925 (impl)
macro_rules! Depcrate_useless_concatimpl_10925 {
() => {
// Module: crate::useless_concat
// Provides: {"impl_10925"}
// Dependencies: {}
impl LateLintPass < '_ > for UselessConcat { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if expr . span . from_expansion () && let ExprKind :: Lit (lit) = expr . kind && let LitKind :: Str (lit_s , _) = lit . node && let Some (macro_call) = macro_backtrace (expr . span) . next () && cx . tcx . is_diagnostic_item (sym :: macro_concat , macro_call . def_id) && let Some (original_code) = snippet_opt (cx , macro_call . span) && let mut parts = original_code . split ('!') && parts . next () . is_some_and (| p | p . trim () == "concat") && parts . next () . is_some_and (| p | p . trim () . starts_with ('(')) { let mut literal = None ; let mut nb_commas = 0 ; let mut nb_idents = 0 ; for (token_kind , token_s , _) in tokenize_with_text (& original_code) { match token_kind { TokenKind :: Eof => break , TokenKind :: Literal { .. } => { if literal . is_some () { return ; } literal = Some (token_s) ; } , TokenKind :: Ident => { if token_s == "true" || token_s == "false" { literal = Some (token_s) ; } else { nb_idents += 1 ; } } , TokenKind :: Comma => { nb_commas += 1 ; if nb_commas > 1 { return ; } } , TokenKind :: Dollar => return , _ => { } , } } if nb_idents == 1 { span_lint_and_sugg (cx , USELESS_CONCAT , macro_call . span , "unneeded use of `concat!` macro" , "replace with" , format ! ("{lit_s:?}") , Applicability :: MachineApplicable ,) ; } } } }
};
}
