// Generated macro for write_warning (function)
macro_rules! Depcrate_command_helperswrite_warning {
() => {
// Module: crate::command_helpers
// Provides: {"write_warning"}
// Dependencies: {}
fn write_warning (line : & [u8]) { let stdout = io :: stdout () ; let mut stdout = stdout . lock () ; stdout . write_all (b"cargo:warning=") . unwrap () ; stdout . write_all (line) . unwrap () ; stdout . write_all (b"\n") . unwrap () ; }
};
}
