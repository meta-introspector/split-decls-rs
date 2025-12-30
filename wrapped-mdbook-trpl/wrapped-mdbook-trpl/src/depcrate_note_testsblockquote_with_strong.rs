// Generated macro for blockquote_with_strong (function)
macro_rules! Depcrate_note_testsblockquote_with_strong {
() => {
// Module: crate::note::tests
// Provides: {"blockquote_with_strong"}
// Dependencies: {}
# [test] fn blockquote_with_strong () { let text = "> **Bold text in a paragraph.**" ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<blockquote>\n<p><strong>Bold text in a paragraph.</strong></p>\n</blockquote>\n") ; }
};
}
