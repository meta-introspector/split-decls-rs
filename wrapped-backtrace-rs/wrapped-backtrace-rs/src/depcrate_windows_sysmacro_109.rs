// Generated macro for macro_109 (macro)
macro_rules! Depcrate_windows_sysmacro_109 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_109"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn MapViewOfFile (hfilemappingobject : HANDLE , dwdesiredaccess : FILE_MAP , dwfileoffsethigh : u32 , dwfileoffsetlow : u32 , dwnumberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS) ;
};
}
