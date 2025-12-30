// Generated macro for impl_108 (impl)
macro_rules! Depcrate_inferimpl_108 {
() => {
// Module: crate::infer
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'db > Index < ExprOrPatId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , id : ExprOrPatId) -> & Ty < 'db > { match id { ExprOrPatId :: ExprId (id) => & self [id] , ExprOrPatId :: PatId (id) => & self [id] , } } }
};
}
