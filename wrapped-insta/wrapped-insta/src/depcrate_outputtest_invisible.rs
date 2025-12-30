// Generated macro for test_invisible (function)
macro_rules! Depcrate_outputtest_invisible {
() => {
// Module: crate::output
// Provides: {"test_invisible"}
// Dependencies: {}
# [test] fn test_invisible () { assert_eq ! (render_invisible ("\r\n\x1b\r\x07\x08\x7f\n" , true) , "␍␊\r\n␛␍\r␇␈␡␊\n") ; }
};
}
