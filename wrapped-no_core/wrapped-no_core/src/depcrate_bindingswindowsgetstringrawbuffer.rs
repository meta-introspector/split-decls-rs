// Generated macro for WindowsGetStringRawBuffer (function)
macro_rules! Depcrate_bindingsWindowsGetStringRawBuffer {
() => {
// Module: crate::bindings
// Provides: {"WindowsGetStringRawBuffer"}
// Dependencies: {}
# [inline] pub unsafe fn WindowsGetStringRawBuffer (string : & windows_strings :: HSTRING , length : Option < * mut u32 > ,) -> windows_strings :: PCWSTR { windows_link :: link ! ("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer (string : * mut core :: ffi :: c_void , length : * mut u32) -> windows_strings :: PCWSTR) ; unsafe { WindowsGetStringRawBuffer (core :: mem :: transmute_copy (string) , length . unwrap_or (core :: mem :: zeroed ()) as _ ,) } }
};
}
