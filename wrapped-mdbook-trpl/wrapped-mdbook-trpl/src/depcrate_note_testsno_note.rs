// Generated macro for no_note (function)
macro_rules! Depcrate_note_testsno_note {
() => {
// Module: crate::note::tests
// Provides: {"no_note"}
// Dependencies: {}
# [test] fn no_note () { let text = "Hello, world.\n\nThis is some text." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<p>Hello, world.</p>\n<p>This is some text.</p>\n") ; }
};
}
