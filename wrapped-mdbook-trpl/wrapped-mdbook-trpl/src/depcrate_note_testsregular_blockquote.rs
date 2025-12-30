// Generated macro for regular_blockquote (function)
macro_rules! Depcrate_note_testsregular_blockquote {
() => {
// Module: crate::note::tests
// Provides: {"regular_blockquote"}
// Dependencies: {}
# [test] fn regular_blockquote () { let text = "> This is some text.\n> It keeps going." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<blockquote>\n<p>This is some text.\nIt keeps going.</p>\n</blockquote>\n") ; }
};
}
