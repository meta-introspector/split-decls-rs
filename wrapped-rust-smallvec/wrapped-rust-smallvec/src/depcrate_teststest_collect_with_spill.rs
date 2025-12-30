// Generated macro for test_collect_with_spill (function)
macro_rules! Depcrate_teststest_collect_with_spill {
() => {
// Module: crate::tests
// Provides: {"test_collect_with_spill"}
// Dependencies: {}
# [test] fn test_collect_with_spill () { let input = "0123456" ; let collected : SmallVec < char , 4 > = input . chars () . collect () ; assert_eq ! (collected , & ['0' , '1' , '2' , '3' , '4' , '5' , '6']) ; }
};
}
