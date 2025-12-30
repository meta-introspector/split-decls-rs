// Generated macro for impl_106 (impl)
macro_rules! Depcrate_inferimpl_106 {
() => {
// Module: crate::infer
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'db > Index < ExprId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , expr : ExprId) -> & Ty < 'db > { self . type_of_expr . get (expr) . unwrap_or (& self . error_ty) } }
};
}
