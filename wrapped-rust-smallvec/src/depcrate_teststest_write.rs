// Generated macro for test_write (function)
macro_rules! Depcrate_teststest_write {
() => {
// Module: crate::tests
// Provides: {"test_write"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] fn test_write () { use std :: io :: Write ; let data = [1 , 2 , 3 , 4 , 5] ; let mut small_vec : SmallVec < u8 , 2 > = SmallVec :: new () ; let len = small_vec . write (& data [..]) . unwrap () ; assert_eq ! (len , 5) ; assert_eq ! (small_vec . as_ref () , data . as_ref ()) ; let mut small_vec : SmallVec < u8 , 2 > = SmallVec :: new () ; small_vec . write_all (& data [..]) . unwrap () ; assert_eq ! (small_vec . as_ref () , data . as_ref ()) ; }
};
}
