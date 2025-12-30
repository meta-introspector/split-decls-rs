// Generated macro for trim_span (function)
macro_rules! Depcrate_sourcetrim_span {
() => {
// Module: crate::source
// Provides: {"trim_span"}
// Dependencies: {}
# [doc = " Trims the whitespace from the start and the end of the span."] pub fn trim_span (sm : & SourceMap , span : Span) -> Span { let data = span . data () ; let sf : & _ = & sm . lookup_source_file (data . lo) ; let Some (src) = sf . src . as_deref () else { return span ; } ; let Some (snip) = & src . get ((data . lo - sf . start_pos) . to_usize () .. (data . hi - sf . start_pos) . to_usize ()) else { return span ; } ; let trim_start = snip . len () - snip . trim_start () . len () ; let trim_end = snip . len () - snip . trim_end () . len () ; SpanData { lo : data . lo + BytePos :: from_usize (trim_start) , hi : data . hi - BytePos :: from_usize (trim_end) , ctxt : data . ctxt , parent : data . parent , } . span () }
};
}
