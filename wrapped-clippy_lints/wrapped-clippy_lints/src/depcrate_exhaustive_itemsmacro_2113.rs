// Generated macro for macro_2113 (macro)
macro_rules! Depcrate_exhaustive_itemsmacro_2113 {
() => {
// Module: crate::exhaustive_items
// Provides: {"macro_2113"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns on any exported `enum`s that are not tagged `#[non_exhaustive]`"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Making an `enum` exhaustive is a stability commitment: adding a variant is a breaking change."] # [doc = " A project may wish to ensure that there are no exhaustive enums or that every exhaustive"] # [doc = " `enum` is explicitly `#[allow]`ed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum Foo {"] # [doc = "     Bar,"] # [doc = "     Baz"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[non_exhaustive]"] # [doc = " enum Foo {"] # [doc = "     Bar,"] # [doc = "     Baz"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub EXHAUSTIVE_ENUMS , restriction , "detects exported enums that have not been marked #[non_exhaustive]" }
};
}
