// Generated macro for from_elem (function)
macro_rules! Depcrate_vecfrom_elem {
() => {
// Module: crate::vec
// Provides: {"from_elem"}
// Dependencies: {}
# [inline (always)] # [cfg (not (no_global_oom_handling))] # [doc (hidden)] pub fn from_elem < T : Clone > (elem : T , n : usize) -> Vec < T > { let mut v = Vec :: with_capacity (n) ; v . extend_with (n , ExtendElement (elem)) ; v }
};
}
