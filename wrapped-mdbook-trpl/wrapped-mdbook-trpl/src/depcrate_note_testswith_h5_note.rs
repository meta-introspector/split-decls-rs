// Generated macro for with_h5_note (function)
macro_rules! Depcrate_note_testswith_h5_note {
() => {
// Module: crate::note::tests
// Provides: {"with_h5_note"}
// Dependencies: {}
# [test] fn with_h5_note () { let text = "> ##### Header\n > And then some note content." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<h5>Header</h5>\n<p>And then some note content.</p>\n</section>") ; }
};
}
