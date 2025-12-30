// Generated macro for integer_const (function)
macro_rules! Depcrate_constsinteger_const {
() => {
// Module: crate::consts
// Provides: {"integer_const"}
// Dependencies: {}
# [doc = " If `expr` evaluates to an integer constant, return its value."] # [doc = ""] # [doc = " The context argument is the context used to view the evaluated expression. e.g. when evaluating"] # [doc = " the argument in `f(m!(1))` the context of the call expression should be used. This is need so"] # [doc = " the const evaluator can see the `m` macro and marke the evaluation as non-local independant of"] # [doc = " what the macro expands to."] pub fn integer_const (cx : & LateContext < '_ > , expr : & Expr < '_ > , ctxt : SyntaxContext) -> Option < u128 > { if let Some (Constant :: Int (value)) = ConstEvalCtxt :: new (cx) . eval_local (expr , ctxt) { Some (value) } else { None } }
};
}
