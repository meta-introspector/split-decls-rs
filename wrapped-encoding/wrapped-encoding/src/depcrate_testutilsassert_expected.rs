// Generated macro for assert_expected (macro)
macro_rules! Depcrate_testutilsassert_expected {
() => {
// Module: crate::testutils
// Provides: {"assert_expected"}
// Dependencies: {}
macro_rules ! assert_expected { ($ result : expr , $ func : expr , $ filter : expr) => ({ use testutils :: Testable ; match $ result { result => { assert ! (result . expected_return == result . actual_return , "{} should return {:?}, but instead returned {:?}" , $ func , $ filter (result . expected_return) , $ filter (result . actual_return)) ; assert ! (& result . expected_push [..] == & result . actual_push [..] , "{} should push {:?}, but instead pushed {:?}" , $ func , result . expected_push , result . actual_push) ; } } }) ; }
};
}
