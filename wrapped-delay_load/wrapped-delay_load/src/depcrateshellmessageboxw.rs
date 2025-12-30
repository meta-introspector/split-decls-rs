// Generated macro for ShellMessageBoxW (type)
macro_rules! DepcrateShellMessageBoxW {
() => {
// Module: crate
// Provides: {"ShellMessageBoxW"}
// Dependencies: {}
type ShellMessageBoxW = unsafe extern "C" fn (happinst : usize , hwnd : usize , lpctext : PCWSTR , lpctitle : PCWSTR , fustyle : u32 , ...) -> i32 ;
};
}
