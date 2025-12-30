// Generated macro for format_arg_removal_span (function)
macro_rules! Depcrate_macrosformat_arg_removal_span {
() => {
// Module: crate::macros
// Provides: {"format_arg_removal_span"}
// Dependencies: {}
# [doc = " Returns the [`Span`] of the value at `index` extended to the previous comma, e.g. for the value"] # [doc = " `10`"] # [doc = ""] # [doc = " ```ignore"] # [doc = " format(\"{}.{}\", 10, 11)"] # [doc = " //            ^^^^"] # [doc = " ```"] pub fn format_arg_removal_span (format_args : & FormatArgs , index : usize) -> Option < Span > { let ctxt = format_args . span . ctxt () ; let current = hygiene :: walk_chain (format_args . arguments . by_index (index) ? . expr . span , ctxt) ; let prev = if index == 0 { format_args . span } else { hygiene :: walk_chain (format_args . arguments . by_index (index - 1) ? . expr . span , ctxt) } ; Some (current . with_lo (prev . hi ())) }
};
}
