// Generated macro for impl_1945 (impl)
macro_rules! Depcrate_vecimpl_1945 {
() => {
// Module: crate::vec
// Provides: {"impl_1945"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Copy , A : Allocator > ExtendFromWithinSpec for Vec < T , A > { unsafe fn spec_extend_from_within (& mut self , src : Range < usize >) { let count = src . len () ; { let (init , spare) = self . split_at_spare_mut () ; let source = unsafe { init . get_unchecked (src) } ; unsafe { ptr :: copy_nonoverlapping (source . as_ptr () , spare . as_mut_ptr () as _ , count) } ; } self . len += count ; } }
};
}
