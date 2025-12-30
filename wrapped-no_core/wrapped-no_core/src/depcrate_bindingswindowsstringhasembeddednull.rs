// Generated macro for WindowsStringHasEmbeddedNull (function)
macro_rules! Depcrate_bindingsWindowsStringHasEmbeddedNull {
() => {
// Module: crate::bindings
// Provides: {"WindowsStringHasEmbeddedNull"}
// Dependencies: {}
# [inline] pub unsafe fn WindowsStringHasEmbeddedNull (string : & windows_strings :: HSTRING ,) -> windows_result :: Result < windows_result :: BOOL > { windows_link :: link ! ("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull (string : * mut core :: ffi :: c_void , hasembednull : * mut windows_result :: BOOL) -> windows_result :: HRESULT) ; unsafe { let mut result__ = core :: mem :: zeroed () ; WindowsStringHasEmbeddedNull (core :: mem :: transmute_copy (string) , & mut result__) . map (| | result__) } }
};
}
