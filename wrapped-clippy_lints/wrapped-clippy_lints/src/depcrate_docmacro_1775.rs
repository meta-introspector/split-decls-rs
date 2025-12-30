// Generated macro for macro_1775 (macro)
macro_rules! Depcrate_docmacro_1775 {
() => {
// Module: crate::doc
// Provides: {"macro_1775"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if the first paragraph in the documentation of items listed in the module page is too long."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Documentation will show the first paragraph of the docstring in the summary page of a"] # [doc = " module. Having a nice, short summary in the first paragraph is part of writing good docs."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " /// A very short summary."] # [doc = " /// A much longer explanation that goes into a lot more detail about"] # [doc = " /// how the thing works, possibly with doclinks and so one,"] # [doc = " /// and probably spanning a many rows."] # [doc = " struct Foo {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// A very short summary."] # [doc = " ///"] # [doc = " /// A much longer explanation that goes into a lot more detail about"] # [doc = " /// how the thing works, possibly with doclinks and so one,"] # [doc = " /// and probably spanning a many rows."] # [doc = " struct Foo {}"] # [doc = " ```"] # [clippy :: version = "1.82.0"] pub TOO_LONG_FIRST_DOC_PARAGRAPH , nursery , "ensure the first documentation paragraph is short" }
};
}
