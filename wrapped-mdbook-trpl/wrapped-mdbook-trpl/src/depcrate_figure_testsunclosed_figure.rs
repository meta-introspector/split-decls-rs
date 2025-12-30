// Generated macro for unclosed_figure (function)
macro_rules! Depcrate_figure_testsunclosed_figure {
() => {
// Module: crate::figure::tests
// Provides: {"unclosed_figure"}
// Dependencies: {}
# [test] fn unclosed_figure () { let result = rewrite_figure ("<figure>") ; let actual = format ! ("{:?}" , result . unwrap_err ()) ; assert_eq ! (actual , "Unclosed `<figure>`") ; }
};
}
