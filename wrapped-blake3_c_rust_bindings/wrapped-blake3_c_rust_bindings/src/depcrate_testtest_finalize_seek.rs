// Generated macro for test_finalize_seek (function)
macro_rules! Depcrate_testtest_finalize_seek {
() => {
// Module: crate::test
// Provides: {"test_finalize_seek"}
// Dependencies: {}
# [test] fn test_finalize_seek () { let mut expected = [0 ; 1000] ; { let mut reference_hasher = reference_impl :: Hasher :: new () ; reference_hasher . update (b"foobarbaz") ; reference_hasher . finalize (& mut expected) ; } let mut test_hasher = crate :: Hasher :: new () ; test_hasher . update (b"foobarbaz") ; let mut out = [0 ; 103] ; for & seek in & [0 , 1 , 7 , 59 , 63 , 64 , 65 , 501 , expected . len () - out . len ()] { dbg ! (seek) ; test_hasher . finalize_seek (seek as u64 , & mut out) ; assert_eq ! (& expected [seek ..] [.. out . len ()] , & out [..]) ; } }
};
}
