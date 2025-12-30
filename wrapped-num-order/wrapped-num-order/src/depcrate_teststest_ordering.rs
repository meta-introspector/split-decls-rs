// Generated macro for test_ordering (function)
macro_rules! Depcrate_teststest_ordering {
() => {
// Module: crate::tests
// Provides: {"test_ordering"}
// Dependencies: {}
# [test] fn test_ordering () { let numbers : Vec < _ > = NUMBERS . iter () . map (| cls | expand_equiv_class (cls)) . collect () ; for icls in 0 .. numbers . len () { for jcls in 0 .. numbers . len () { let expected = icls . cmp (& jcls) ; for i in & numbers [icls] { for j in & numbers [jcls] { assert_cmp (i , j , expected) ; } } } } }
};
}
