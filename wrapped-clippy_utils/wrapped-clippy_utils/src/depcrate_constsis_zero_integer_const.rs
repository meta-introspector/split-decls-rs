// Generated macro for is_zero_integer_const (function)
macro_rules! Depcrate_constsis_zero_integer_const {
() => {
// Module: crate::consts
// Provides: {"is_zero_integer_const"}
// Dependencies: {}
# [doc = " Check if `expr` evaluates to an integer constant of 0."] # [doc = ""] # [doc = " The context argument is the context used to view the evaluated expression. e.g. when evaluating"] # [doc = " the argument in `f(m!(1))` the context of the call expression should be used. This is need so"] # [doc = " the const evaluator can see the `m` macro and marke the evaluation as non-local independant of"] # [doc = " what the macro expands to."] # [inline] pub fn is_zero_integer_const (cx : & LateContext < '_ > , expr : & Expr < '_ > , ctxt : SyntaxContext) -> bool { integer_const (cx , expr , ctxt) == Some (0) }
};
}
