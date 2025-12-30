// Generated macro for macro_9396 (macro)
macro_rules! Depcrate_redundant_static_lifetimesmacro_9396 {
() => {
// Module: crate::redundant_static_lifetimes
// Provides: {"macro_9396"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for constants and statics with an explicit `'static` lifetime."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Adding `'static` to every reference can create very"] # [doc = " complicated types."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " const FOO: &'static [(&'static str, &'static str, fn(&Bar) -> bool)] ="] # [doc = " &[...]"] # [doc = " static FOO: &'static [(&'static str, &'static str, fn(&Bar) -> bool)] ="] # [doc = " &[...]"] # [doc = " ```"] # [doc = " This code can be rewritten as"] # [doc = " ```ignore"] # [doc = "  const FOO: &[(&str, &str, fn(&Bar) -> bool)] = &[...]"] # [doc = "  static FOO: &[(&str, &str, fn(&Bar) -> bool)] = &[...]"] # [doc = " ```"] # [clippy :: version = "1.37.0"] pub REDUNDANT_STATIC_LIFETIMES , style , "Using explicit `'static` lifetime for constants or statics when elision rules would allow omitting them." }
};
}
