// Generated macro for unclosed_caption (function)
macro_rules! Depcrate_figure_testsunclosed_caption {
() => {
// Module: crate::figure::tests
// Provides: {"unclosed_caption"}
// Dependencies: {}
# [test] fn unclosed_caption () { let result = rewrite_figure ("<figure>
<figcaption>
</figure>" ,) ; let actual = format ! ("{:?}" , result . unwrap_err ()) ; assert_eq ! (actual , "Unclosed `<figcaption>`") ; }
};
}
