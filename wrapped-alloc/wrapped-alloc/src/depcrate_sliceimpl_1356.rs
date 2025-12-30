// Generated macro for impl_1356 (impl)
macro_rules! Depcrate_sliceimpl_1356 {
() => {
// Module: crate::slice
// Provides: {"impl_1356"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (feature = "slice_concat_ext" , issue = "27747")] impl < T : Clone , V : Borrow < [T] > > Concat < T > for [V] { type Output = Vec < T > ; fn concat (slice : & Self) -> Vec < T > { let size = slice . iter () . map (| slice | slice . borrow () . len ()) . sum () ; let mut result = Vec :: with_capacity (size) ; for v in slice { result . extend_from_slice (v . borrow ()) } result } }
};
}
