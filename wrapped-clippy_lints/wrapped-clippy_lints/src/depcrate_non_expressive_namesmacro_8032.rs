// Generated macro for macro_8032 (macro)
macro_rules! Depcrate_non_expressive_namesmacro_8032 {
() => {
// Module: crate::non_expressive_names
// Provides: {"macro_8032"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if you have variables whose name consists of just"] # [doc = " underscores and digits."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's hard to memorize what a variable means without a"] # [doc = " descriptive name."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _1 = 1;"] # [doc = " let ___1 = 1;"] # [doc = " let __1___2 = 11;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub JUST_UNDERSCORES_AND_DIGITS , style , "unclear name" }
};
}
