// Generated macro for has_span_from_proc_macro (function)
macro_rules! Depcrate_utils_format_args_collectorhas_span_from_proc_macro {
() => {
// Module: crate::utils::format_args_collector
// Provides: {"has_span_from_proc_macro"}
// Dependencies: {}
# [doc = " Detects if the format string or an argument has its span set by a proc macro to something inside"] # [doc = " a macro callsite, e.g."] # [doc = ""] # [doc = " ```ignore"] # [doc = " println!(some_proc_macro!(\"input {}\"), a);"] # [doc = " ```"] # [doc = ""] # [doc = " Where `some_proc_macro` expands to"] # [doc = ""] # [doc = " ```ignore"] # [doc = " println!(\"output {}\", a);"] # [doc = " ```"] # [doc = ""] # [doc = " But with the span of `\"output {}\"` set to the macro input"] # [doc = ""] # [doc = " ```ignore"] # [doc = " println!(some_proc_macro!(\"input {}\"), a);"] # [doc = " //                        ^^^^^^^^^^"] # [doc = " ```"] fn has_span_from_proc_macro (cx : & EarlyContext < '_ > , args : & FormatArgs) -> bool { let ctxt = args . span . ctxt () ; let argument_span = args . arguments . explicit_args () . iter () . map (| argument | hygiene :: walk_chain (argument . expr . span , ctxt)) ; ! once (args . span) . chain (argument_span) . tuple_windows () . map (| (start , end) | start . between (end)) . all (| sp | { sp . check_source_text (cx , | src | { let mut iter = tokenize (src , FrontmatterAllowed :: No) . filter (| t | { ! matches ! (t . kind , TokenKind :: LineComment { .. } | TokenKind :: BlockComment { .. } | TokenKind :: Whitespace) }) ; iter . next () . is_some_and (| t | matches ! (t . kind , TokenKind :: Comma)) && iter . all (| t | matches ! (t . kind , TokenKind :: Ident | TokenKind :: Eq)) }) }) }
};
}
