// Generated macro for macro_17 (macro)
macro_rules! Depcrate_bindingsmacro_17 {
() => {
// Module: crate::bindings
// Provides: {"macro_17"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegEnumKeyExW (hkey : HKEY , dwindex : u32 , lpname : PWSTR , lpcchname : * mut u32 , lpreserved : * const u32 , lpclass : PWSTR , lpcchclass : * mut u32 , lpftlastwritetime : * mut FILETIME) -> WIN32_ERROR) ;
};
}
