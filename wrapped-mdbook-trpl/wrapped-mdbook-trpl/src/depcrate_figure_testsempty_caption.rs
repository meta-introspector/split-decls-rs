// Generated macro for empty_caption (function)
macro_rules! Depcrate_figure_testsempty_caption {
() => {
// Module: crate::figure::tests
// Provides: {"empty_caption"}
// Dependencies: {}
# [test] fn empty_caption () { let result = rewrite_figure ("<figure>
<figcaption></figcaption>
</figure>" ,) ; let actual = format ! ("{:?}" , result . unwrap_err ()) ; assert_eq ! (actual , "Missing caption in `<figcaption>`") ; }
};
}
