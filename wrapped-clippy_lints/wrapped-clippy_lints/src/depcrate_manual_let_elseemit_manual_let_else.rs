// Generated macro for emit_manual_let_else (function)
macro_rules! Depcrate_manual_let_elseemit_manual_let_else {
() => {
// Module: crate::manual_let_else
// Provides: {"emit_manual_let_else"}
// Dependencies: {}
fn emit_manual_let_else (cx : & LateContext < '_ > , span : Span , expr : & Expr < '_ > , ident_map : & FxHashMap < Symbol , (& Pat < '_ > , BindingMode) > , pat : & Pat < '_ > , else_body : & Expr < '_ > ,) { span_lint_and_then (cx , MANUAL_LET_ELSE , span , "this could be rewritten as `let...else`" , | diag | { let mut app = Applicability :: HasPlaceholders ; let (sn_expr , _) = snippet_with_context (cx , expr . span , span . ctxt () , "" , & mut app) ; let (sn_else , else_is_mac_call) = snippet_with_context (cx , else_body . span , span . ctxt () , "" , & mut app) ; let else_bl = if matches ! (else_body . kind , ExprKind :: Block (..)) && ! else_is_mac_call { sn_else . into_owned () } else { format ! ("{{ {sn_else} }}") } ; let sn_bl = replace_in_pattern (cx , span , ident_map , pat , & mut app , true) ; let sugg = if sn_expr . ends_with ('}') { format ! ("let {sn_bl} = ({sn_expr}) else {else_bl};") } else { format ! ("let {sn_bl} = {sn_expr} else {else_bl};") } ; diag . span_suggestion (span , "consider writing" , sugg , app) ; } ,) ; }
};
}
