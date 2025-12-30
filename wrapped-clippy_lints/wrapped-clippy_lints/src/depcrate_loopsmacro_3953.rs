// Generated macro for macro_3953 (macro)
macro_rules! Depcrate_loopsmacro_3953 {
() => {
// Module: crate::loops
// Provides: {"macro_3953"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks whether variables used within while loop condition"] # [doc = " can be (and are) mutated in the body."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If the condition is unchanged, entering the body of the loop"] # [doc = " will lead to an infinite loop."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the `while`-loop is in a closure, the check for mutation of the"] # [doc = " condition variables in the body can cause false negatives. For example when only `Upvar` `a` is"] # [doc = " in the condition and only `Upvar` `b` gets mutated in the body, the lint will not trigger."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let i = 0;"] # [doc = " while i > 10 {"] # [doc = "     println!(\"let me loop forever!\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WHILE_IMMUTABLE_CONDITION , correctness , "variables used within while expression are not mutated in the body" }
};
}
