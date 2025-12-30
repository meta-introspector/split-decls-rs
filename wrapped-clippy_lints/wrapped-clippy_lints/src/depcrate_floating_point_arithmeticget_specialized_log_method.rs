// Generated macro for get_specialized_log_method (function)
macro_rules! Depcrate_floating_point_arithmeticget_specialized_log_method {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"get_specialized_log_method"}
// Dependencies: {}
fn get_specialized_log_method (cx : & LateContext < '_ > , base : & Expr < '_ > , ctxt : SyntaxContext) -> Option < & 'static str > { if let Some (value) = ConstEvalCtxt :: new (cx) . eval_local (base , ctxt) { if F32 (2.0) == value || F64 (2.0) == value { return Some ("log2") ; } else if F32 (10.0) == value || F64 (10.0) == value { return Some ("log10") ; } else if F32 (f32_consts :: E) == value || F64 (f64_consts :: E) == value { return Some ("ln") ; } } None }
};
}
