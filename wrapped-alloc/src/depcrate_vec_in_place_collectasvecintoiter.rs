// Generated macro for AsVecIntoIter (trait)
macro_rules! Depcrate_vec_in_place_collectAsVecIntoIter {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"AsVecIntoIter"}
// Dependencies: {}
# [doc = " Internal helper trait for in-place iteration specialization."] # [doc = ""] # [doc = " Currently this is only implemented by [`vec::IntoIter`] - returning a reference to itself - and"] # [doc = " [`binary_heap::IntoIter`] which returns a reference to its inner representation."] # [doc = ""] # [doc = " Since this is an internal trait it hides the implementation detail `binary_heap::IntoIter`"] # [doc = " uses `vec::IntoIter` internally."] # [doc = ""] # [doc = " [`vec::IntoIter`]: super::IntoIter"] # [doc = " [`binary_heap::IntoIter`]: crate::collections::binary_heap::IntoIter"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " In-place iteration relies on implementation details of `vec::IntoIter`, most importantly that"] # [doc = " it does not create references to the whole allocation during iteration, only raw pointers"] # [rustc_specialization_trait] pub (crate) unsafe trait AsVecIntoIter { type Item ; fn as_into_iter (& mut self) -> & mut super :: IntoIter < Self :: Item > ; }
};
}
