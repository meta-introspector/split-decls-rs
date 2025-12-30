// Generated macro for macro_3906 (macro)
macro_rules! Depcrate_loopsmacro_3906 {
() => {
// Module: crate::loops
// Provides: {"macro_3906"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for loops that check for emptiness of a `Vec` in the condition and pop an element"] # [doc = " in the body as a separate operation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Such loops can be written in a more idiomatic way by using a while-let loop and directly"] # [doc = " pattern matching on the return value of `Vec::pop()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut numbers = vec![1, 2, 3, 4, 5];"] # [doc = " while !numbers.is_empty() {"] # [doc = "     let number = numbers.pop().unwrap();"] # [doc = "     // use `number`"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut numbers = vec![1, 2, 3, 4, 5];"] # [doc = " while let Some(number) = numbers.pop() {"] # [doc = "     // use `number`"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub MANUAL_WHILE_LET_SOME , style , "checking for emptiness of a `Vec` in the loop condition and popping an element in the body" }
};
}
