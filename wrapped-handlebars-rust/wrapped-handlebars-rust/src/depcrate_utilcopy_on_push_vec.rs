// Generated macro for copy_on_push_vec (function)
macro_rules! Depcrate_utilcopy_on_push_vec {
() => {
// Module: crate::util
// Provides: {"copy_on_push_vec"}
// Dependencies: {}
# [inline] pub (crate) fn copy_on_push_vec < T > (input : & [T] , el : T) -> Vec < T > where T : Clone , { let mut new_vec = Vec :: with_capacity (input . len () + 1) ; new_vec . extend_from_slice (input) ; new_vec . push (el) ; new_vec }
};
}
