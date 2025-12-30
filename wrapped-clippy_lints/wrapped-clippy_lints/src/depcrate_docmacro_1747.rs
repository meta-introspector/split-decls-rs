// Generated macro for macro_1747 (macro)
macro_rules! Depcrate_docmacro_1747 {
() => {
// Module: crate::doc
// Provides: {"macro_1747"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `fn main() { .. }` in doctests"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The test can be shorter (and likely more readable)"] # [doc = " if the `fn main()` is left implicit."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " /// An example of a doctest with a `main()` function"] # [doc = " ///"] # [doc = " /// # Examples"] # [doc = " ///"] # [doc = " /// ```"] # [doc = " /// fn main() {"] # [doc = " ///     // this needs not be in an `fn`"] # [doc = " /// }"] # [doc = " /// ```"] # [doc = " fn needless_main() {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub NEEDLESS_DOCTEST_MAIN , style , "presence of `fn main() {` in code examples" }
};
}
