// Generated macro for macro_8743 (macro)
macro_rules! Depcrate_operatorsmacro_8743 {
() => {
// Module: crate::operators
// Provides: {"macro_8743"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for equal operands to comparison, logical and"] # [doc = " bitwise, difference and division binary operators (`==`, `>`, etc., `&&`,"] # [doc = " `||`, `&`, `|`, `^`, `-` and `/`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is usually just a typo or a copy and paste error."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False negatives: We had some false positives regarding"] # [doc = " calls (notably [racer](https://github.com/phildawes/racer) had one instance"] # [doc = " of `x.pop() && x.pop()`), so we removed matching any function or method"] # [doc = " calls. We may introduce a list of known pure functions in the future."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " if x + 1 == x + 1 {}"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " # let a = 3;"] # [doc = " # let b = 4;"] # [doc = " assert_eq!(a, a);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EQ_OP , correctness , "equal operands on both sides of a comparison or bitwise combination (e.g., `x == x`)" }
};
}
