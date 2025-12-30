// Generated macro for format_args_inputs_span (function)
macro_rules! Depcrate_macrosformat_args_inputs_span {
() => {
// Module: crate::macros
// Provides: {"format_args_inputs_span"}
// Dependencies: {}
# [doc = " Span covering the format string and values"] # [doc = ""] # [doc = " ```ignore"] # [doc = " format(\"{}.{}\", 10, 11)"] # [doc = " //     ^^^^^^^^^^^^^^^"] # [doc = " ```"] pub fn format_args_inputs_span (format_args : & FormatArgs) -> Span { match format_args . arguments . explicit_args () { [] => format_args . span , [.. , last] => format_args . span . to (hygiene :: walk_chain (last . expr . span , format_args . span . ctxt ())) , } }
};
}
