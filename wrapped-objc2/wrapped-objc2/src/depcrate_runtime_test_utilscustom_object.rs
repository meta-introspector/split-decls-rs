// Generated macro for custom_object (function)
macro_rules! Depcrate_runtime_test_utilscustom_object {
() => {
// Module: crate::runtime::test_utils
// Provides: {"custom_object"}
// Dependencies: {}
pub (crate) fn custom_object () -> Retained < CustomObject > { let ptr : * const AnyClass = custom_class () ; unsafe { Retained :: from_raw (ffi :: class_createInstance (ptr , 0) . cast ()) } . unwrap () }
};
}
