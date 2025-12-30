// Generated macro for copy_slice_and_advance (macro)
macro_rules! Depcrate_strcopy_slice_and_advance {
() => {
// Module: crate::str
// Provides: {"copy_slice_and_advance"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] macro_rules ! copy_slice_and_advance { ($ target : expr , $ bytes : expr) => { let len = $ bytes . len () ; let (head , tail) = { $ target } . split_at_mut (len) ; head . copy_from_slice ($ bytes) ; $ target = tail ; } ; }
};
}
