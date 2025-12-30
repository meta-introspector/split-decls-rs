// Generated macro for text_with_figure_replaces_it_with_simple_text (function)
macro_rules! Depcrate_figure_teststext_with_figure_replaces_it_with_simple_text {
() => {
// Module: crate::figure::tests
// Provides: {"text_with_figure_replaces_it_with_simple_text"}
// Dependencies: {}
# [test] fn text_with_figure_replaces_it_with_simple_text () { let actual = rewrite_figure (r#"<figure>

<img src="http://www.example.com/some-image.jpg">

<figcaption>Figure 12-34: Look at this cool picture!</figcaption>

</figure>"# ,) . unwrap () ; let expected = r#"

<img src="http://www.example.com/some-image.jpg">

Figure 12-34: Look at this cool picture!

"# ; assert_eq ! (actual , expected) ; }
};
}
