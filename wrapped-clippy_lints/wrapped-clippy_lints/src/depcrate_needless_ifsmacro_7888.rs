// Generated macro for macro_7888 (macro)
macro_rules! Depcrate_needless_ifsmacro_7888 {
() => {
// Module: crate::needless_ifs
// Provides: {"macro_7888"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty `if` branches with no else branch."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be entirely omitted, and often the condition too."] # [doc = ""] # [doc = " ### Known issues"] # [doc = " This will usually only suggest to remove the `if` statement, not the condition. Other lints"] # [doc = " such as `no_effect` will take care of removing the condition if it's unnecessary."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if really_expensive_condition(&i) {}"] # [doc = " if really_expensive_condition_with_side_effects(&mut i) {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " // <omitted>"] # [doc = " really_expensive_condition_with_side_effects(&mut i);"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NEEDLESS_IFS , complexity , "checks for empty if branches" }
};
}
