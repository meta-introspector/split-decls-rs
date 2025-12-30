// Generated macro for from_elem_in (function)
macro_rules! Depcrate_vecfrom_elem_in {
() => {
// Module: crate::vec
// Provides: {"from_elem_in"}
// Dependencies: {}
# [inline (always)] # [cfg (not (no_global_oom_handling))] # [doc (hidden)] pub fn from_elem_in < T : Clone , A : Allocator > (elem : T , n : usize , alloc : A) -> Vec < T , A > { let mut v = Vec :: with_capacity_in (n , alloc) ; v . extend_with (n , ExtendElement (elem)) ; v }
};
}
