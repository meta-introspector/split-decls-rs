// Generated macro for macro_7257 (macro)
macro_rules! Depcrate_misc_earlymacro_7257 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7257"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `[all @ ..]` patterns."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In all cases, `all` works fine and can often make code simpler, as you possibly won't need"] # [doc = " to convert from say a `Vec` to a slice by dereferencing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if let [all @ ..] = &*v {"] # [doc = "     // NOTE: Type is a slice here"] # [doc = "     println!(\"all elements: {all:#?}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " if let all = v {"] # [doc = "     // NOTE: Type is a `Vec` here"] # [doc = "     println!(\"all elements: {all:#?}\");"] # [doc = " }"] # [doc = " // or"] # [doc = " println!(\"all elements: {v:#?}\");"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub REDUNDANT_AT_REST_PATTERN , complexity , "checks for `[all @ ..]` where `all` would suffice" }
};
}
