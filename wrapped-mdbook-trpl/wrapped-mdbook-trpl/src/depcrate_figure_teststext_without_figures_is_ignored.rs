// Generated macro for text_without_figures_is_ignored (function)
macro_rules! Depcrate_figure_teststext_without_figures_is_ignored {
() => {
// Module: crate::figure::tests
// Provides: {"text_without_figures_is_ignored"}
// Dependencies: {}
# [test] fn text_without_figures_is_ignored () { let actual = rewrite_figure ("This is some basic text.") . unwrap () ; assert_eq ! (actual , "This is some basic text.") ; }
};
}
