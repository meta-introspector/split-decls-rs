// Generated macro for with_h6_note (function)
macro_rules! Depcrate_note_testswith_h6_note {
() => {
// Module: crate::note::tests
// Provides: {"with_h6_note"}
// Dependencies: {}
# [test] fn with_h6_note () { let text = "> ###### Header\n > And then some note content." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<h6>Header</h6>\n<p>And then some note content.</p>\n</section>") ; }
};
}
