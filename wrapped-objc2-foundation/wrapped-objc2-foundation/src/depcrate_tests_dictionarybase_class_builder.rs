// Generated macro for base_class_builder (function)
macro_rules! Depcrate_tests_dictionarybase_class_builder {
() => {
// Module: crate::tests::dictionary
// Provides: {"base_class_builder"}
// Dependencies: {}
fn base_class_builder (name : & str) -> Option < ClassBuilder > { extern "C-unwind" fn initialize (_cls : & AnyClass , _sel : Sel) { } let mut builder = ClassBuilder :: root (& CString :: new (name) . unwrap () , initialize as extern "C-unwind" fn (_ , _) ,) ? ; extern "C-unwind" fn new (cls : & AnyClass , _sel : Sel) -> * mut AnyObject { unsafe { ffi :: class_createInstance (cls , 0) } } unsafe { builder . add_class_method (sel ! (new) , new as extern "C-unwind" fn (_ , _) -> _) ; } extern "C-unwind" fn does_not_recognize_selector (obj : & AnyObject , _sel : Sel , sel : Sel) { panic ! ("does not recognize: -[{} {sel}]" , obj . class ()) ; } unsafe { builder . add_method (sel ! (doesNotRecognizeSelector :) , does_not_recognize_selector as extern "C-unwind" fn (_ , _ , _) ,) ; } extern "C-unwind" fn forward_invocation (obj : & AnyObject , _sel : Sel , invocation : & AnyObject) { let sel : Sel = unsafe { msg_send ! [invocation , selector] } ; panic ! ("does not recognize: -[{} {sel}]" , obj . class ()) ; } unsafe { builder . add_method (sel ! (forwardInvocation :) , forward_invocation as extern "C-unwind" fn (_ , _ , _) ,) ; } extern "C-unwind" fn release (obj : * mut AnyObject , _sel : Sel) { # [allow (deprecated)] unsafe { ffi :: object_dispose (obj . cast ()) ; } } unsafe { builder . add_method (sel ! (release) , release as extern "C-unwind" fn (_ , _)) ; } Some (builder) }
};
}
