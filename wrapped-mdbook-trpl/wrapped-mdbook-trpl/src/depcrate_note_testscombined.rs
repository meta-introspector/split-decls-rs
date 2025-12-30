// Generated macro for combined (function)
macro_rules! Depcrate_note_testscombined {
() => {
// Module: crate::note::tests
// Provides: {"combined"}
// Dependencies: {}
# [test] fn combined () { let text = "> Note: This is some text.\n> It keeps going.\n\nThis is regular text.\n\n> This is a blockquote.\n" ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is some text.\nIt keeps going.</p>\n</section>\n<p>This is regular text.</p>\n<blockquote>\n<p>This is a blockquote.</p>\n</blockquote>\n") ; }
};
}
