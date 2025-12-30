// Generated macro for blockquote_then_note (function)
macro_rules! Depcrate_note_testsblockquote_then_note {
() => {
// Module: crate::note::tests
// Provides: {"blockquote_then_note"}
// Dependencies: {}
# [test] fn blockquote_then_note () { let text = "> This is quoted.\n\n> Note: This is noted." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n<section class=\"note\" aria-role=\"note\">\n<p>Note: This is noted.</p>\n</section>") ; }
};
}
