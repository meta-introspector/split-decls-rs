// Generated macro for macro_8535 (macro)
macro_rules! Depcrate_operatorsmacro_8535 {
() => {
// Module: crate::operators
// Provides: {"macro_8535"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for arguments to `==` which have their address"] # [doc = " taken to satisfy a bound"] # [doc = " and suggests to dereference the other argument instead"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is more idiomatic to dereference the other argument."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " &x == y"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " x == *y"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OP_REF , style , "taking a reference to satisfy the type constraints on `==`" }
};
}
