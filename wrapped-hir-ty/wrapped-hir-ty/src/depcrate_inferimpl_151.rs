// Generated macro for impl_151 (impl)
macro_rules! Depcrate_inferimpl_151 {
() => {
// Module: crate::infer
// Provides: {"impl_151"}
// Dependencies: {}
impl Index < BindingId > for InferenceResult { type Output = Ty ; fn index (& self , b : BindingId) -> & Ty { self . type_of_binding . get (b) . unwrap_or (& self . standard_types . unknown) } }
};
}
