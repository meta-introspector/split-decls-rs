// Generated macro for SysFreeString (function)
macro_rules! Depcrate_bindingsSysFreeString {
() => {
// Module: crate::bindings
// Provides: {"SysFreeString"}
// Dependencies: {}
# [inline] pub unsafe fn SysFreeString (bstrstring : & windows_core :: BSTR) { windows_core :: link ! ("oleaut32.dll" "system" fn SysFreeString (bstrstring : * mut core :: ffi :: c_void)) ; unsafe { SysFreeString (core :: mem :: transmute_copy (bstrstring)) } }
};
}
