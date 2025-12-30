// Generated macro for slice (function)
macro_rules! Depcrate_primitivesslice {
() => {
// Module: crate::primitives
// Provides: {"slice"}
// Dependencies: {}
# [test] fn slice () { let slice : & [bool] = & [] ; assert_eq ! (Bake :: bake (& slice , & Default :: default ()) . to_string () , "& []") ; assert_eq ! (BakeSize :: borrows_size (& slice) , 0) ; let slice : & [bool] = & [true] ; assert_eq ! (Bake :: bake (& slice , & Default :: default ()) . to_string () , "& [true]" ,) ; assert_eq ! (BakeSize :: borrows_size (& slice) , 1) ; let slice : & [bool] = & [true , false] ; assert_eq ! (Bake :: bake (& slice , & Default :: default ()) . to_string () , "& [true , false]" ,) ; assert_eq ! (BakeSize :: borrows_size (& slice) , 2) ; let slice : & [u8] = b"hello" ; assert_eq ! (Bake :: bake (& slice , & Default :: default ()) . to_string () , r#"b"hello""# ,) ; assert_eq ! (BakeSize :: borrows_size (& slice) , 5) ; }
};
}
