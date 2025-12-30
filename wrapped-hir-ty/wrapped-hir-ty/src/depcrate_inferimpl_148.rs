// Generated macro for impl_148 (impl)
macro_rules! Depcrate_inferimpl_148 {
() => {
// Module: crate::infer
// Provides: {"impl_148"}
// Dependencies: {}
impl Index < ExprId > for InferenceResult { type Output = Ty ; fn index (& self , expr : ExprId) -> & Ty { self . type_of_expr . get (expr) . unwrap_or (& self . standard_types . unknown) } }
};
}
