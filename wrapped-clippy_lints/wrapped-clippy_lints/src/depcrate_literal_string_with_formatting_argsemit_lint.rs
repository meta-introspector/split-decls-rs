// Generated macro for emit_lint (function)
macro_rules! Depcrate_literal_string_with_formatting_argsemit_lint {
() => {
// Module: crate::literal_string_with_formatting_args
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , expr : & Expr < '_ > , spans : & [(Span , Option < String >)]) { if ! spans . is_empty () && let Some (mir) = enclosing_mir (cx . tcx , expr . hir_id) { let spans = spans . iter () . filter_map (| (span , name) | { if let Some (name) = name && ! mir . var_debug_info . iter () . any (| local | ! local . source_info . span . from_expansion () && local . name . as_str () == name) { return None ; } Some (* span) }) . collect :: < Vec < _ > > () ; match spans . len () { 0 => { } , 1 => { span_lint (cx , LITERAL_STRING_WITH_FORMATTING_ARGS , spans , "this looks like a formatting argument but it is not part of a formatting macro" ,) ; } , _ => { span_lint (cx , LITERAL_STRING_WITH_FORMATTING_ARGS , spans , "these look like formatting arguments but are not part of a formatting macro" ,) ; } , } } }
};
}
