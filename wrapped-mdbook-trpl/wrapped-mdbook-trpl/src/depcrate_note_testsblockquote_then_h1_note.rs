// Generated macro for blockquote_then_h1_note (function)
macro_rules! Depcrate_note_testsblockquote_then_h1_note {
() => {
// Module: crate::note::tests
// Provides: {"blockquote_then_h1_note"}
// Dependencies: {}
# [test] fn blockquote_then_h1_note () { let text = "> This is quoted.\n\n> # Header\n > And then some note content." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n<section class=\"note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>") ; }
};
}
