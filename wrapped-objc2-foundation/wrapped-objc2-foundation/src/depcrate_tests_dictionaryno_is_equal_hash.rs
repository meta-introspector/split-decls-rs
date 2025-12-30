// Generated macro for no_is_equal_hash (function)
macro_rules! Depcrate_tests_dictionaryno_is_equal_hash {
() => {
// Module: crate::tests::dictionary
// Provides: {"no_is_equal_hash"}
// Dependencies: {}
# [test] # [should_panic = "does not recognize: -[NoIsEqualHash hash]"] # [cfg (feature = "NSZone")] # [cfg_attr (feature = "gnustep-1-7" , ignore = "GNUStep stack overflows here for some reason?")] # [cfg_attr (all (target_os = "macos" , target_arch = "x86") , ignore = "the old runtime seems to have been compiled with -fno-exceptions?")] fn no_is_equal_hash () { let mut builder = base_class_builder ("NoIsEqualHash") . unwrap () ; extern "C-unwind" fn copy_with_zone (obj : & AnyObject , _sel : Sel , _zone : * mut crate :: NSZone ,) -> * mut AnyObject { unsafe { ffi :: class_createInstance (obj . class () , 0) } } unsafe { builder . add_method (sel ! (copyWithZone :) , copy_with_zone as extern "C-unwind" fn (_ , _ , _) -> _ ,) ; } let cls = builder . register () ; test_from_base_class (cls) ; }
};
}
