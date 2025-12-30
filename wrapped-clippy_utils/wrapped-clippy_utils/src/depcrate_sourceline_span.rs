// Generated macro for line_span (function)
macro_rules! Depcrate_sourceline_span {
() => {
// Module: crate::source
// Provides: {"line_span"}
// Dependencies: {}
# [doc = " Extends the span to the beginning of the spans line, incl. whitespaces."] # [doc = ""] # [doc = " ```no_run"] # [doc = "        let x = ();"] # [doc = " //             ^^"] # [doc = " // will be converted to"] # [doc = "        let x = ();"] # [doc = " // ^^^^^^^^^^^^^^"] # [doc = " ```"] fn line_span (sess : & impl HasSession , span : Span) -> Span { let span = original_sp (span , DUMMY_SP) ; let SourceFileAndLine { sf , line } = sess . sess () . source_map () . lookup_line (span . lo ()) . unwrap () ; let line_start = sf . lines () [line] ; let line_start = sf . absolute_position (line_start) ; span . with_lo (line_start) }
};
}
