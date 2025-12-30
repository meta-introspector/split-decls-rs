// Generated macro for array (function)
macro_rules! Depcrate_primitivesarray {
() => {
// Module: crate::primitives
// Provides: {"array"}
// Dependencies: {}
# [test] fn array () { test_bake ! ([bool ; 0] , const , []) ; test_bake ! ([bool ; 1] , const , [true]) ; test_bake ! ([bool ; 2] , const , [true , false]) ; assert_eq ! (BakeSize :: borrows_size (& ["hello" , "world"]) , 10) ; test_bake ! ([u8 ; 5] , const , * b"hello") ; assert_eq ! (BakeSize :: borrows_size (b"hello") , 0) ; }
};
}
