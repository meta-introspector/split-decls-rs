// Generated macro for impl_1358 (impl)
macro_rules! Depcrate_sliceimpl_1358 {
() => {
// Module: crate::slice
// Provides: {"impl_1358"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (feature = "slice_concat_ext" , issue = "27747")] impl < T : Clone , V : Borrow < [T] > > Join < & [T] > for [V] { type Output = Vec < T > ; fn join (slice : & Self , sep : & [T]) -> Vec < T > { let mut iter = slice . iter () ; let first = match iter . next () { Some (first) => first , None => return vec ! [] , } ; let size = slice . iter () . map (| v | v . borrow () . len ()) . sum :: < usize > () + sep . len () * (slice . len () - 1) ; let mut result = Vec :: with_capacity (size) ; result . extend_from_slice (first . borrow ()) ; for v in iter { result . extend_from_slice (sep) ; result . extend_from_slice (v . borrow ()) } result } }
};
}
