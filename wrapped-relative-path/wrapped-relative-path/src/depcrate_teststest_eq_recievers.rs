// Generated macro for test_eq_recievers (function)
macro_rules! Depcrate_teststest_eq_recievers {
() => {
// Module: crate::tests
// Provides: {"test_eq_recievers"}
// Dependencies: {}
# [test] fn test_eq_recievers () { use std :: borrow :: Cow ; let borrowed : & RelativePath = RelativePath :: new ("foo/bar") ; let mut owned : RelativePathBuf = RelativePathBuf :: new () ; owned . push ("foo") ; owned . push ("bar") ; let borrowed_cow : Cow < RelativePath > = borrowed . into () ; let owned_cow : Cow < RelativePath > = owned . clone () . into () ; macro_rules ! t { ($ ($ current : expr) ,+) => { $ (assert_eq ! ($ current , borrowed) ; assert_eq ! ($ current , owned) ; assert_eq ! ($ current , borrowed_cow) ; assert_eq ! ($ current , owned_cow) ;) + } } t ! (borrowed , owned , borrowed_cow , owned_cow) ; }
};
}
