// Generated macro for expand_past_previous_comma (function)
macro_rules! Depcrate_sourceexpand_past_previous_comma {
() => {
// Module: crate::source
// Provides: {"expand_past_previous_comma"}
// Dependencies: {}
# [doc = " Expand a span to include a preceding comma"] # [doc = " ```rust,ignore"] # [doc = " writeln!(o, \"\")   ->   writeln!(o, \"\")"] # [doc = "             ^^                   ^^^^"] # [doc = " ```"] pub fn expand_past_previous_comma (sess : & impl HasSession , span : Span) -> Span { let extended = sess . sess () . source_map () . span_extend_to_prev_char (span , ',' , true) ; extended . with_lo (extended . lo () - BytePos (1)) }
};
}
