// Generated macro for impl_149 (impl)
macro_rules! Depcrate_inferimpl_149 {
() => {
// Module: crate::infer
// Provides: {"impl_149"}
// Dependencies: {}
impl Index < PatId > for InferenceResult { type Output = Ty ; fn index (& self , pat : PatId) -> & Ty { self . type_of_pat . get (pat) . unwrap_or (& self . standard_types . unknown) } }
};
}
