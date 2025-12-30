// Generated macro for note_then_blockquote (function)
macro_rules! Depcrate_note_testsnote_then_blockquote {
() => {
// Module: crate::note::tests
// Provides: {"note_then_blockquote"}
// Dependencies: {}
# [test] fn note_then_blockquote () { let text = "> Note: This is noted.\n\n> This is quoted." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is noted.</p>\n</section>\n<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n") ; }
};
}
