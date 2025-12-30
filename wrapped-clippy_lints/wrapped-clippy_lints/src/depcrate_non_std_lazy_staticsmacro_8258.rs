// Generated macro for macro_8258 (macro)
macro_rules! Depcrate_non_std_lazy_staticsmacro_8258 {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"macro_8258"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints when `once_cell::sync::Lazy` or `lazy_static!` are used to define a static variable,"] # [doc = " and suggests replacing such cases with `std::sync::LazyLock` instead."] # [doc = ""] # [doc = " Note: This lint will not trigger in crate with `no_std` context, or with MSRV < 1.80.0. It"] # [doc = " also will not trigger on `once_cell::sync::Lazy` usage in crates which use other types"] # [doc = " from `once_cell`, such as `once_cell::race::OnceBox`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " - Reduces the need for an extra dependency"] # [doc = " - Enforce convention of using standard library types when possible"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " lazy_static! {"] # [doc = "     static ref FOO: String = \"foo\".to_uppercase();"] # [doc = " }"] # [doc = " static BAR: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| \"BAR\".to_lowercase());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " static FOO: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| \"FOO\".to_lowercase());"] # [doc = " static BAR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| \"BAR\".to_lowercase());"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub NON_STD_LAZY_STATICS , pedantic , "lazy static that could be replaced by `std::sync::LazyLock`" }
};
}
