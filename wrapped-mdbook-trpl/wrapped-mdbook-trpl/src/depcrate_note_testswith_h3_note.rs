// Generated macro for with_h3_note (function)
macro_rules! Depcrate_note_testswith_h3_note {
() => {
// Module: crate::note::tests
// Provides: {"with_h3_note"}
// Dependencies: {}
# [test] fn with_h3_note () { let text = "> ### Header\n > And then some note content." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<h3>Header</h3>\n<p>And then some note content.</p>\n</section>") ; }
};
}
