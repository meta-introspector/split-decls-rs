// Generated macro for multiple_niches_test (function)
macro_rules! Depcrate_testsmultiple_niches_test {
() => {
// Module: crate::tests
// Provides: {"multiple_niches_test"}
// Dependencies: {}
# [rustversion :: since (1.65)] # [test] fn multiple_niches_test () { # [allow (unused)] enum Value { String (CompactString) , Bool (bool) , Signed (isize) , Unsigned (usize) , Null , } assert_eq ! (core :: mem :: size_of ::< Value > () , core :: mem :: size_of ::< String > ()) ; }
};
}
