// Generated macro for with_note (function)
macro_rules! Depcrate_note_testswith_note {
() => {
// Module: crate::note::tests
// Provides: {"with_note"}
// Dependencies: {}
# [test] fn with_note () { let text = "> Note: This is some text.\n> It keeps going." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is some text.\nIt keeps going.</p>\n</section>") ; }
};
}
