// Generated macro for is_exe (function)
macro_rules! Depcrateis_exe {
() => {
// Module: crate
// Provides: {"is_exe"}
// Dependencies: {}
fn is_exe (executable : & Path) -> bool { executable . extension () == Some (std :: ffi :: OsStr :: new ("exe")) }
};
}
