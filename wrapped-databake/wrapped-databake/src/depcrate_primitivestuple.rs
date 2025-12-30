// Generated macro for tuple (function)
macro_rules! Depcrate_primitivestuple {
() => {
// Module: crate::primitives
// Provides: {"tuple"}
// Dependencies: {}
# [test] fn tuple () { test_bake ! (() , const , ()) ; assert_eq ! (BakeSize :: borrows_size (& ()) , 0) ; test_bake ! ((u8 ,) , const , (0u8 ,)) ; assert_eq ! (BakeSize :: borrows_size (& ("hi" ,)) , 2) ; test_bake ! ((u8 , i8) , const , (0u8 , 0i8)) ; assert_eq ! (BakeSize :: borrows_size (& ("hi" , 8u8)) , 2) ; }
};
}
