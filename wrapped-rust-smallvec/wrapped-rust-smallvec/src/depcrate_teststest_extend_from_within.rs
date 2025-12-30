// Generated macro for test_extend_from_within (function)
macro_rules! Depcrate_teststest_extend_from_within {
() => {
// Module: crate::tests
// Provides: {"test_extend_from_within"}
// Dependencies: {}
# [test] fn test_extend_from_within () { let mut v : SmallVec < u8 , 8 > = smallvec ! [0 , 1 , 2 , 3] ; v . extend_from_within (1 .. 3) ; assert_eq ! (& v . iter () . map (| v | * v) . collect ::< Vec < _ >> () , & [0 , 1 , 2 , 3 , 1 , 2] ,) ; }
};
}
