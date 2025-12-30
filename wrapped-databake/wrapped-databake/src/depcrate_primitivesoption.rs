// Generated macro for option (function)
macro_rules! Depcrate_primitivesoption {
() => {
// Module: crate::primitives
// Provides: {"option"}
// Dependencies: {}
# [test] fn option () { test_bake ! (Option <&'static str >, const , Some ("hello")) ; assert_eq ! (BakeSize :: borrows_size (& Some ("hello")) , 5) ; test_bake ! (Option <&'static str >, const , None) ; assert_eq ! (BakeSize :: borrows_size (& None ::<&'static str >) , 0) ; }
};
}
