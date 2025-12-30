// Generated macro for test_constructor (function)
macro_rules! Depcratetest_constructor {
() => {
// Module: crate
// Provides: {"test_constructor"}
// Dependencies: {}
# [test] fn test_constructor () { let test = unsafe { bindings :: Test :: new (5) } ; assert_eq ! (test . m_int , 5) ; assert_eq ! (test . m_double , 0.0) ; }
};
}
