// Generated macro for result_opt (macro)
macro_rules! Depcrate_exprresult_opt {
() => {
// Module: crate::expr
// Provides: {"result_opt"}
// Dependencies: {}
macro_rules ! result_opt ((fn $ n : ident : $ e : ident -> $ t : ty) => (# [allow (dead_code)] # [allow (clippy :: wrong_self_convention)] fn $ n (self) -> Option <$ t > { if let EvalResult ::$ e (v) = self { Some (v) } else { None } }) ;) ;
};
}
