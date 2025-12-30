// Generated macro for test (module)
macro_rules! Depcrate_timetest {
() => {
// Module: crate::time
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (test , feature = "utest"))] mod test { use super :: * ; # [test] fn check_rdtsc () { let cpuid = crate :: cpuid :: CpuId :: new () ; let has_tsc = cpuid . get_feature_info () . map_or (false , | finfo | finfo . has_tsc ()) ; if has_tsc { unsafe { assert ! (rdtsc () > 0 , "rdtsc returned 0, unlikely!") ; } } } # [test] fn check_rdtscp () { let cpuid = crate :: cpuid :: CpuId :: new () ; let has_rdtscp = cpuid . get_extended_processor_and_feature_identifiers () . map_or (false , | einfo | einfo . has_rdtscp ()) ; if has_rdtscp { unsafe { assert ! (rdtscp () . 0 > 0 , "rdtscp returned 0, unlikely!") ; if cfg ! (target_os = "linux") { let mut cpu : u32 = 0 ; let mut node : u32 = 0 ; libc :: syscall (libc :: SYS_getcpu , & mut cpu , & mut node , 0) ; assert_eq ! (rdtscp () . 1 , node << 12 | cpu , "rdtscp AUX didn't match getcpu call!") ; } } } } }
};
}
