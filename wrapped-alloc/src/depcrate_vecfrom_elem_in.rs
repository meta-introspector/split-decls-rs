// Generated macro for from_elem_in (function)
macro_rules! Depcrate_vecfrom_elem_in {
() => {
// Module: crate::vec
// Provides: {"from_elem_in"}
// Dependencies: {}
# [doc (hidden)] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "allocator_api" , issue = "32838")] # [track_caller] pub fn from_elem_in < T : Clone , A : Allocator > (elem : T , n : usize , alloc : A) -> Vec < T , A > { < T as SpecFromElem > :: from_elem (elem , n , alloc) }
};
}
