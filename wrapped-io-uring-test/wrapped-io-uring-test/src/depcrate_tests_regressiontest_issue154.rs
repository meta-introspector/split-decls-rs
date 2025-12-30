// Generated macro for test_issue154 (function)
macro_rules! Depcrate_tests_regressiontest_issue154 {
() => {
// Module: crate::tests::regression
// Provides: {"test_issue154"}
// Dependencies: {}
pub fn test_issue154 < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (_ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; } println ! ("test issue #154") ; let err = match IoUring :: new (u32 :: MAX) { Ok (_) => panic ! () , Err (err) => err , } ; assert_eq ! (err . kind () , std :: io :: ErrorKind :: InvalidInput) ; Ok (()) }
};
}
