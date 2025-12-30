// Generated macro for macro_9448 (macro)
macro_rules! Depcrate_ref_option_refmacro_9448 {
() => {
// Module: crate::ref_option_ref
// Provides: {"macro_9448"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `&Option<&T>`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since `&` is Copy, it's useless to have a"] # [doc = " reference on `Option<&T>`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " It may be irrelevant to use this lint on"] # [doc = " public API code as it will make a breaking change to apply it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let x: &Option<&u32> = &Some(&0u32);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let x: Option<&u32> = Some(&0u32);"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub REF_OPTION_REF , pedantic , "use `Option<&T>` instead of `&Option<&T>`" }
};
}
