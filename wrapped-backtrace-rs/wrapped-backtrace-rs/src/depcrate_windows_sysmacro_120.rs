// Generated macro for macro_120 (macro)
macro_rules! Depcrate_windows_sysmacro_120 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_120"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn WideCharToMultiByte (codepage : u32 , dwflags : u32 , lpwidecharstr : PCWSTR , cchwidechar : i32 , lpmultibytestr : PSTR , cbmultibyte : i32 , lpdefaultchar : PCSTR , lpuseddefaultchar : * mut BOOL) -> i32) ;
};
}
