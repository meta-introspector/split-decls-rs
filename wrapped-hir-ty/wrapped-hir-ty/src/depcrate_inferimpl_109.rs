// Generated macro for impl_109 (impl)
macro_rules! Depcrate_inferimpl_109 {
() => {
// Module: crate::infer
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'db > Index < BindingId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , b : BindingId) -> & Ty < 'db > { self . type_of_binding . get (b) . unwrap_or (& self . error_ty) } }
};
}
