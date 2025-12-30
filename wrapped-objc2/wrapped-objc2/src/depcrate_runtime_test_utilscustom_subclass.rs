// Generated macro for custom_subclass (function)
macro_rules! Depcrate_runtime_test_utilscustom_subclass {
() => {
// Module: crate::runtime::test_utils
// Provides: {"custom_subclass"}
// Dependencies: {}
pub (crate) fn custom_subclass () -> & 'static AnyClass { static REGISTER_CUSTOM_SUBCLASS : Once = Once :: new () ; REGISTER_CUSTOM_SUBCLASS . call_once (| | { let superclass = custom_class () ; let mut builder = ClassBuilder :: new (& c ("CustomSubclassObject") , superclass) . unwrap () ; extern "C-unwind" fn custom_subclass_get_foo (this : & AnyObject , _cmd : Sel) -> u32 { let foo : u32 = unsafe { msg_send ! [super (this , custom_class ()) , foo] } ; foo + 2 } extern "C-unwind" fn custom_subclass_class_method (_cls : & AnyClass , _cmd : Sel) -> u32 { 9 } unsafe { let get_foo : extern "C-unwind" fn (_ , _) -> _ = custom_subclass_get_foo ; builder . add_method (sel ! (foo) , get_foo) ; let class_method : extern "C-unwind" fn (_ , _) -> _ = custom_subclass_class_method ; builder . add_class_method (sel ! (classFoo) , class_method) ; } builder . register () ; }) ; AnyClass :: get (& c ("CustomSubclassObject")) . unwrap () }
};
}
