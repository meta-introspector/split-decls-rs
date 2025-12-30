// Generated macro for macro_14 (macro)
macro_rules! Depcrate_imp_bindingsmacro_14 {
() => {
// Module: crate::imp::bindings
// Provides: {"macro_14"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryExA (lplibfilename : PCSTR , hfile : HANDLE , dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE) ;
};
}
