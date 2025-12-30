// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (test , feature = "utest"))] mod test { use super :: * ; # [test] fn test_rdpid () { let rdpid_support = cpuid :: CpuId :: new () . get_extended_feature_info () . map_or (false , | finfo | finfo . has_rdpid ()) ; unsafe { if rdpid_support { let pid1 = rdpid () ; let pid2 = rdpid () ; assert ! (pid1 == pid2 , "RDPID not consistent values?") ; } } } }
};
}
