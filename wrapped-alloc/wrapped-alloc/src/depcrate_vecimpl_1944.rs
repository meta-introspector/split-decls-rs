// Generated macro for impl_1944 (impl)
macro_rules! Depcrate_vecimpl_1944 {
() => {
// Module: crate::vec
// Provides: {"impl_1944"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator > ExtendFromWithinSpec for Vec < T , A > { default unsafe fn spec_extend_from_within (& mut self , src : Range < usize >) { let (this , spare , len) = unsafe { self . split_at_spare_mut_with_len () } ; let to_clone = unsafe { this . get_unchecked (src) } ; iter :: zip (to_clone , spare) . map (| (src , dst) | dst . write (src . clone ())) . for_each (| _ | * len += 1) ; } }
};
}
