// Generated macro for no_copy (function)
macro_rules! Depcrate_tests_dictionaryno_copy {
() => {
// Module: crate::tests::dictionary
// Provides: {"no_copy"}
// Dependencies: {}
# [test] # [should_panic = "does not recognize: -[NoCopy copyWithZone:]"] # [cfg_attr (feature = "gnustep-1-7" , ignore = "GNUStep stack overflows here for some reason?")] # [cfg_attr (all (target_os = "macos" , target_arch = "x86") , ignore = "the old runtime seems to have been compiled with -fno-exceptions?")] fn no_copy () { let mut builder = base_class_builder ("NoCopy") . unwrap () ; extern "C-unwind" fn hash (obj : & AnyObject , _sel : Sel) -> NSUInteger { obj as * const AnyObject as NSUInteger } unsafe { builder . add_method (sel ! (hash) , hash as extern "C-unwind" fn (_ , _) -> _) ; } extern "C-unwind" fn is_equal (obj : & AnyObject , _sel : Sel , other : & AnyObject) -> Bool { ptr :: eq (obj , other) . into () } unsafe { builder . add_method (sel ! (isEqual :) , is_equal as extern "C-unwind" fn (_ , _ , _) -> _ ,) ; } let cls = builder . register () ; test_from_base_class (cls) ; }
};
}
