// Generated macro for test_retain (function)
macro_rules! Depcrate_teststest_retain {
() => {
// Module: crate::tests
// Provides: {"test_retain"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_retain () { let mut s = CompactString :: from ("α_β_γ") ; s . retain (| _ | true) ; assert_eq ! (s , "α_β_γ") ; s . retain (| c | c != '_') ; assert_eq ! (s , "αβγ") ; s . retain (| c | c != 'β') ; assert_eq ! (s , "αγ") ; s . retain (| c | c == 'α') ; assert_eq ! (s , "α") ; s . retain (| _ | false) ; assert_eq ! (s , "") ; let mut s = CompactString :: from ("0è0") ; let _ = std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (| | { let mut count = 0 ; s . retain (| _ | { count += 1 ; match count { 1 => false , 2 => true , _ => panic ! () , } }) ; })) ; assert ! (std :: str :: from_utf8 (s . as_bytes ()) . is_ok ()) ; }
};
}
