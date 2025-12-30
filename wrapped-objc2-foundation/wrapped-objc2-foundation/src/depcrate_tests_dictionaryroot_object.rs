// Generated macro for root_object (function)
macro_rules! Depcrate_tests_dictionaryroot_object {
() => {
// Module: crate::tests::dictionary
// Provides: {"root_object"}
// Dependencies: {}
# [test] # [cfg (feature = "NSZone")] # [cfg_attr (feature = "gnustep-1-7" , ignore = "GNUStep stack overflows here for some reason?")] fn root_object () { let mut builder = base_class_builder ("RootObject") . unwrap () ; extern "C-unwind" fn copy_with_zone (obj : & AnyObject , _sel : Sel , _zone : * mut crate :: NSZone ,) -> * mut AnyObject { unsafe { msg_send ! [obj . class () , new] } } unsafe { builder . add_method (sel ! (copyWithZone :) , copy_with_zone as extern "C-unwind" fn (_ , _ , _) -> _ ,) ; } extern "C-unwind" fn hash (obj : & AnyObject , _sel : Sel) -> NSUInteger { obj as * const AnyObject as NSUInteger } unsafe { builder . add_method (sel ! (hash) , hash as extern "C-unwind" fn (_ , _) -> _) ; } extern "C-unwind" fn is_equal (obj : & AnyObject , _sel : Sel , other : & AnyObject) -> Bool { ptr :: eq (obj , other) . into () } unsafe { builder . add_method (sel ! (isEqual :) , is_equal as extern "C-unwind" fn (_ , _ , _) -> _ ,) ; } let cls = builder . register () ; test_from_base_class (cls) ; }
};
}
