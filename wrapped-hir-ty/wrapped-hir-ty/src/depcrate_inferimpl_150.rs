// Generated macro for impl_150 (impl)
macro_rules! Depcrate_inferimpl_150 {
() => {
// Module: crate::infer
// Provides: {"impl_150"}
// Dependencies: {}
impl Index < ExprOrPatId > for InferenceResult { type Output = Ty ; fn index (& self , id : ExprOrPatId) -> & Ty { self . type_of_expr_or_pat (id) . unwrap_or (& self . standard_types . unknown) } }
};
}
