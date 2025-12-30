// Generated macro for macro_1777 (macro)
macro_rules! Depcrate_docmacro_1777 {
() => {
// Module: crate::doc
// Provides: {"macro_1777"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if a link reference definition appears at the start of a"] # [doc = " list item or quote."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably intended as an intra-doc link. If it is really"] # [doc = " supposed to be a reference definition, it can be written outside"] # [doc = " of the list item or quote."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " //! - [link]: description"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " //! - [link][]: description (for intra-doc link)"] # [doc = " //!"] # [doc = " //! [link]: destination (for link reference definition)"] # [doc = " ```"] # [clippy :: version = "1.85.0"] pub DOC_NESTED_REFDEFS , suspicious , "link reference defined in list item or quote" }
};
}
