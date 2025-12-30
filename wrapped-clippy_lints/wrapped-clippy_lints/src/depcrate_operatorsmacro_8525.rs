// Generated macro for macro_8525 (macro)
macro_rules! Depcrate_operatorsmacro_8525 {
() => {
// Module: crate::operators
// Provides: {"macro_8525"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `a = a op b` or `a = b commutative_op a`"] # [doc = " patterns."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These can be written as the shorter `a op= b`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " While forbidden by the spec, `OpAssign` traits may have"] # [doc = " implementations that differ from the regular `Op` impl."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut a = 5;"] # [doc = " let b = 0;"] # [doc = " // ..."] # [doc = ""] # [doc = " a = a + b;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut a = 5;"] # [doc = " let b = 0;"] # [doc = " // ..."] # [doc = ""] # [doc = " a += b;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ASSIGN_OP_PATTERN , style , "assigning the result of an operation on a variable to that same variable" }
};
}
