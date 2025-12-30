// Generated macro for macro_2752 (macro)
macro_rules! Depcrate_ifsmacro_2752 {
() => {
// Module: crate::ifs
// Provides: {"macro_2752"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if the `if` and `else` block contain shared code that can be"] # [doc = " moved out of the blocks."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Duplicate code is less maintainable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let foo = if … {"] # [doc = "     println!(\"Hello World\");"] # [doc = "     13"] # [doc = " } else {"] # [doc = "     println!(\"Hello World\");"] # [doc = "     42"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " println!(\"Hello World\");"] # [doc = " let foo = if … {"] # [doc = "     13"] # [doc = " } else {"] # [doc = "     42"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub BRANCHES_SHARING_CODE , nursery , "`if` statement with shared code in all blocks" }
};
}
