// Generated macro for h1_then_blockquote (function)
macro_rules! Depcrate_note_testsh1_then_blockquote {
() => {
// Module: crate::note::tests
// Provides: {"h1_then_blockquote"}
// Dependencies: {}
# [test] fn h1_then_blockquote () { let text = "> # Header\n > And then some note content.\n\n> This is quoted." ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>\n<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n") ; }
};
}
