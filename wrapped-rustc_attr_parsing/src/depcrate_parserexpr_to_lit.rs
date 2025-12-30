// Generated macro for expr_to_lit (function)
macro_rules! Depcrate_parserexpr_to_lit {
() => {
// Module: crate::parser
// Provides: {"expr_to_lit"}
// Dependencies: {}
fn expr_to_lit (psess : & ParseSess , expr : & Expr , span : Span , should_emit : ShouldEmit ,) -> Option < MetaItemLit > { if let ExprKind :: Lit (token_lit) = expr . kind { let res = MetaItemLit :: from_token_lit (token_lit , expr . span) ; match res { Ok (lit) => { if token_lit . suffix . is_some () { should_emit . emit_err (psess . dcx () . create_err (SuffixedLiteralInAttribute { span : lit . span }) ,) ; None } else { if ! lit . kind . is_unsuffixed () { should_emit . emit_err (psess . dcx () . create_err (SuffixedLiteralInAttribute { span : lit . span }) ,) ; } Some (lit) } } Err (err) => { let guar = report_lit_error (psess , err , token_lit , expr . span) ; let lit = MetaItemLit { symbol : token_lit . symbol , suffix : token_lit . suffix , kind : LitKind :: Err (guar) , span : expr . span , } ; Some (lit) } } } else { if matches ! (should_emit , ShouldEmit :: Nothing) { return None ; } let msg = "attribute value must be a literal" ; let err = psess . dcx () . struct_span_err (span , msg) ; should_emit . emit_err (err) ; None } }
};
}
