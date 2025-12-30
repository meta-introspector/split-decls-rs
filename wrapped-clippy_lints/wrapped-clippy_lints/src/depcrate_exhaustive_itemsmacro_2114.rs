// Generated macro for macro_2114 (macro)
macro_rules! Depcrate_exhaustive_itemsmacro_2114 {
() => {
// Module: crate::exhaustive_items
// Provides: {"macro_2114"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns on any exported `struct`s that are not tagged `#[non_exhaustive]`"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Making a `struct` exhaustive is a stability commitment: adding a field is a breaking change."] # [doc = " A project may wish to ensure that there are no exhaustive structs or that every exhaustive"] # [doc = " `struct` is explicitly `#[allow]`ed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo {"] # [doc = "     bar: u8,"] # [doc = "     baz: String,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[non_exhaustive]"] # [doc = " struct Foo {"] # [doc = "     bar: u8,"] # [doc = "     baz: String,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub EXHAUSTIVE_STRUCTS , restriction , "detects exported structs that have not been marked #[non_exhaustive]" }
};
}
