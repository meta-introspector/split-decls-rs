// Generated macro for test_hash (function)
macro_rules! Depcrate_teststest_hash {
() => {
// Module: crate::tests
// Provides: {"test_hash"}
// Dependencies: {}
# [test] fn test_hash () { for & equiv in NUMBERS { let hashes : Vec < u64 > = equiv . iter () . map (| n | hash (n)) . collect () ; for i in 1 .. equiv . len () { assert_eq ! (hashes [0] , hashes [i] , "Hash mismatch between {:?} and {:?}" , equiv [0] , equiv [i]) ; } } }
};
}
