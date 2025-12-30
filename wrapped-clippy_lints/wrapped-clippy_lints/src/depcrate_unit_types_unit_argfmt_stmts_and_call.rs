// Generated macro for fmt_stmts_and_call (function)
macro_rules! Depcrate_unit_types_unit_argfmt_stmts_and_call {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"fmt_stmts_and_call"}
// Dependencies: {}
fn fmt_stmts_and_call (cx : & LateContext < '_ > , call_expr : & Expr < '_ > , call_snippet : & str , args_snippets : Vec < Sugg < '_ > > , non_empty_block_args_snippets : Vec < MaybeTypeUncertain < '_ > > ,) -> String { let call_expr_indent = indent_of (cx , call_expr . span) . unwrap_or (0) ; let call_snippet_with_replacements = args_snippets . into_iter () . fold (call_snippet . to_owned () , | acc , arg | { acc . replacen (& arg . to_string () , "()" , 1) }) ; let stmts_and_call = non_empty_block_args_snippets . into_iter () . map (Into :: into) . chain (iter :: once (call_snippet_with_replacements)) . map (| v | reindent_multiline (& v , true , Some (call_expr_indent))) . collect :: < Vec < _ > > () ; let mut stmts_and_call_snippet = stmts_and_call . join (& format ! ("{}{}" , ";\n" , " " . repeat (call_expr_indent))) ; let parent_node = cx . tcx . parent_hir_node (call_expr . hir_id) ; if ! matches ! (parent_node , Node :: Block (_)) && ! matches ! (parent_node , Node :: Stmt (_)) { let block_indent = call_expr_indent + 4 ; stmts_and_call_snippet = reindent_multiline (& stmts_and_call_snippet , true , Some (block_indent)) ; stmts_and_call_snippet = format ! ("{{\n{}{}\n{}}}" , " " . repeat (block_indent) , & stmts_and_call_snippet , " " . repeat (call_expr_indent)) ; } stmts_and_call_snippet }
};
}
