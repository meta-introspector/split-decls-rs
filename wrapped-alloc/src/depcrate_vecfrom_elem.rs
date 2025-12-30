// Generated macro for from_elem (function)
macro_rules! Depcrate_vecfrom_elem {
() => {
// Module: crate::vec
// Provides: {"from_elem"}
// Dependencies: {}
# [doc (hidden)] # [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_diagnostic_item = "vec_from_elem"] # [track_caller] pub fn from_elem < T : Clone > (elem : T , n : usize) -> Vec < T > { < T as SpecFromElem > :: from_elem (elem , n , Global) }
};
}
