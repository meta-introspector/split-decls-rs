// Generated macro for impl_107 (impl)
macro_rules! Depcrate_inferimpl_107 {
() => {
// Module: crate::infer
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'db > Index < PatId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , pat : PatId) -> & Ty < 'db > { self . type_of_pat . get (pat) . unwrap_or (& self . error_ty) } }
};
}
