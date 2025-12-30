// Generated macro for custom_subclass_object (function)
macro_rules! Depcrate_runtime_test_utilscustom_subclass_object {
() => {
// Module: crate::runtime::test_utils
// Provides: {"custom_subclass_object"}
// Dependencies: {}
pub (crate) fn custom_subclass_object () -> Retained < CustomObject > { let ptr : * const AnyClass = custom_subclass () ; unsafe { Retained :: from_raw (ffi :: class_createInstance (ptr , 0) . cast ()) } . unwrap () }
};
}
