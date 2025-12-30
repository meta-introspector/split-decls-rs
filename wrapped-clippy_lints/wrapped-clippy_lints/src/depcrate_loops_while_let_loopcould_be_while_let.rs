// Generated macro for could_be_while_let (function)
macro_rules! Depcrate_loops_while_let_loopcould_be_while_let {
() => {
// Module: crate::loops::while_let_loop
// Provides: {"could_be_while_let"}
// Dependencies: {}
fn could_be_while_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , let_pat : & 'tcx Pat < '_ > , let_expr : & 'tcx Expr < '_ > , has_trailing_exprs : bool , let_info : Option < (& Pat < '_ > , Option < & Ty < '_ > >) > , inner_expr : Option < & Expr < '_ > > ,) { if has_trailing_exprs && (needs_ordered_drop (cx , cx . typeck_results () . expr_ty (let_expr)) || any_temporaries_need_ordered_drop (cx , let_expr)) { return ; } let inner_content = if let Some (((pat , ty) , inner_expr)) = let_info . zip (inner_expr) && (! is_trivial_assignment (pat , peel_blocks (inner_expr)) || ty . is_some ()) && let Some (pat_str) = snippet_opt (cx , pat . span) && let Some (init_str) = snippet_opt (cx , peel_blocks (inner_expr) . span) { let ty_str = ty . map (| ty | format ! (": {}" , snippet (cx , ty . span , "_"))) . unwrap_or_default () ; format ! ("\n{indent}    let {pat_str}{ty_str} = {init_str};\n{indent}    ..\n{indent}" , indent = snippet_indent (cx , expr . span) . unwrap_or_default () ,) } else { " .. " . into () } ; span_lint_and_sugg (cx , WHILE_LET_LOOP , expr . span , "this loop could be written as a `while let` loop" , "try" , format ! ("while let {} = {} {{{inner_content}}}" , snippet (cx , let_pat . span , "..") , snippet (cx , let_expr . span , "..") ,) , Applicability :: HasPlaceholders ,) ; }
};
}
