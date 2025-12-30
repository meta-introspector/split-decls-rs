// Generated macro for inner_check (function)
macro_rules! Depcrate_zero_repeat_side_effectsinner_check {
() => {
// Module: crate::zero_repeat_side_effects
// Provides: {"inner_check"}
// Dependencies: {}
fn inner_check (cx : & LateContext < '_ > , expr : & '_ rustc_hir :: Expr < '_ > , inner_expr : & '_ rustc_hir :: Expr < '_ > , is_vec : bool) { if inner_expr . can_have_side_effects () { let parent_hir_node = cx . tcx . parent_hir_node (expr . hir_id) ; let inner_expr_ty = cx . typeck_results () . expr_ty (inner_expr) ; let return_type = cx . typeck_results () . expr_ty (expr) ; let inner_expr = snippet (cx , inner_expr . span . source_callsite () , "..") ; let indent = snippet_indent (cx , expr . span) . unwrap_or_default () ; let vec = if is_vec { "vec!" } else { "" } ; let (span , sugg) = match parent_hir_node { Node :: LetStmt (l) => (l . span , format ! ("{inner_expr};\n{indent}let {var_name}: {return_type} = {vec}[];" , var_name = snippet (cx , l . pat . span . source_callsite () , "..")) ,) , Node :: Expr (x) if let ExprKind :: Assign (l , _ , _) = x . kind => (x . span , format ! ("{inner_expr};\n{indent}{var_name} = {vec}[] as {return_type}" , var_name = snippet (cx , l . span . source_callsite () , "..")) ,) , Node :: Stmt (_) => (expr . span , format ! ("{inner_expr};\n{indent}{vec}[] as {return_type}")) , _ => (expr . span , format ! ("\
{{
{indent}    {inner_expr};
{indent}    {vec}[] as {return_type}
{indent}}}") ,) , } ; let span = span . source_callsite () ; span_lint_and_then (cx , ZERO_REPEAT_SIDE_EFFECTS , span , "expression with side effects as the initial value in a zero-sized array initializer" , | diag | { if (! inner_expr_ty . is_never () || cx . tcx . features () . never_type ()) && return_type . is_suggestable (cx . tcx , true) { diag . span_suggestion_verbose (span , "consider performing the side effect separately" , sugg , Applicability :: Unspecified ,) ; } else { diag . help ("consider performing the side effect separately") ; } } ,) ; } }
};
}
