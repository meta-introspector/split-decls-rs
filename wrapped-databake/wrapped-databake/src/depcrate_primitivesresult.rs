// Generated macro for result (function)
macro_rules! Depcrate_primitivesresult {
() => {
// Module: crate::primitives
// Provides: {"result"}
// Dependencies: {}
# [test] fn result () { test_bake ! (Result <&'static str , () >, const , Ok ("hello")) ; assert_eq ! (BakeSize :: borrows_size (& Ok ::< _ , () > ("hello")) , 5) ; test_bake ! (Result <&'static str , () >, const , Err (())) ; assert_eq ! (BakeSize :: borrows_size (& Err ::<&'static str , _ > ("hi")) , 2) ; }
};
}
