// Generated macro for macro_3907 (macro)
macro_rules! Depcrate_loopsmacro_3907 {
() => {
// Module: crate::loops
// Provides: {"macro_3907"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for infinite loops in a function where the return type is not `!`"] # [doc = " and lint accordingly."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Making the return type `!` serves as documentation that the function does not return."] # [doc = " If the function is not intended to loop infinitely, then this lint may detect a bug."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run,ignore"] # [doc = " fn run_forever() {"] # [doc = "     loop {"] # [doc = "         // do something"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " If infinite loops are as intended:"] # [doc = " ```no_run,ignore"] # [doc = " fn run_forever() -> ! {"] # [doc = "     loop {"] # [doc = "         // do something"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Otherwise add a `break` or `return` condition:"] # [doc = " ```no_run,ignore"] # [doc = " fn run_forever() {"] # [doc = "     loop {"] # [doc = "         // do something"] # [doc = "         if condition {"] # [doc = "             break;"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub INFINITE_LOOP , restriction , "possibly unintended infinite loop" }
};
}
