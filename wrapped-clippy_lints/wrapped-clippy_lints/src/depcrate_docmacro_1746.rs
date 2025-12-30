// Generated macro for macro_1746 (macro)
macro_rules! Depcrate_docmacro_1746 {
() => {
// Module: crate::doc
// Provides: {"macro_1746"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks the doc comments of publicly visible functions that"] # [doc = " may panic and warns if there is no `# Panics` section."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Documenting the scenarios in which panicking occurs"] # [doc = " can help callers who do not want to panic to avoid those situations."] # [doc = ""] # [doc = " ### Examples"] # [doc = " Since the following function may panic it has a `# Panics` section in"] # [doc = " its doc comment:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " /// # Panics"] # [doc = " ///"] # [doc = " /// Will panic if y is 0"] # [doc = " pub fn divide_by(x: i32, y: i32) -> i32 {"] # [doc = "     if y == 0 {"] # [doc = "         panic!(\"Cannot divide by 0\")"] # [doc = "     } else {"] # [doc = "         x / y"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Individual panics within a function can be ignored with `#[expect]` or"] # [doc = " `#[allow]`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use std::num::NonZeroUsize;"] # [doc = " pub fn will_not_panic(x: usize) {"] # [doc = "     #[expect(clippy::missing_panics_doc, reason = \"infallible\")]"] # [doc = "     let y = NonZeroUsize::new(1).unwrap();"] # [doc = ""] # [doc = "     // If any panics are added in the future the lint will still catch them"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub MISSING_PANICS_DOC , pedantic , "`pub fn` may panic without `# Panics` in doc comment" }
};
}
