// Generated macro for macro_101 (macro)
macro_rules! Depcrate_windows_sysmacro_101 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_101"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn CreateFileMappingA (hfile : HANDLE , lpfilemappingattributes : * const SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : PCSTR) -> HANDLE) ;
};
}
