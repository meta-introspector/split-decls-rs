// Generated macro for macro_9080 (macro)
macro_rules! Depcrate_redundant_elsemacro_9080 {
() => {
// Module: crate::redundant_else
// Provides: {"macro_9080"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `else` blocks that can be removed without changing semantics."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `else` block adds unnecessary indentation and verbosity."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Some may prefer to keep the `else` block for clarity."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn my_func(count: u32) {"] # [doc = "     if count == 0 {"] # [doc = "         print!(\"Nothing to do\");"] # [doc = "         return;"] # [doc = "     } else {"] # [doc = "         print!(\"Moving on...\");"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn my_func(count: u32) {"] # [doc = "     if count == 0 {"] # [doc = "         print!(\"Nothing to do\");"] # [doc = "         return;"] # [doc = "     }"] # [doc = "     print!(\"Moving on...\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub REDUNDANT_ELSE , pedantic , "`else` branch that can be removed without changing semantics" }
};
}
