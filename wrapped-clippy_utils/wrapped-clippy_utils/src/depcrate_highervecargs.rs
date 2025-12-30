// Generated macro for VecArgs (enum)
macro_rules! Depcrate_higherVecArgs {
() => {
// Module: crate::higher
// Provides: {"VecArgs"}
// Dependencies: {}
# [doc = " Represents the pre-expansion arguments of a `vec!` invocation."] pub enum VecArgs < 'a > { # [doc = " `vec![elem; len]`"] Repeat (& 'a Expr < 'a > , & 'a Expr < 'a >) , # [doc = " `vec![a, b, c]`"] Vec (& 'a [Expr < 'a >]) , }
};
}
