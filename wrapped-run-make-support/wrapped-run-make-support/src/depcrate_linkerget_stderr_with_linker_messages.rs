// Generated macro for get_stderr_with_linker_messages (function)
macro_rules! Depcrate_linkerget_stderr_with_linker_messages {
() => {
// Module: crate::linker
// Provides: {"get_stderr_with_linker_messages"}
// Dependencies: {}
fn get_stderr_with_linker_messages (rustc : & mut Rustc) -> String { let linker_version_flag = if is_windows_msvc () { "--version" } else { "-Wl,-v" } ; let output = rustc . arg ("-Wlinker-messages") . link_arg (linker_version_flag) . run () ; output . stderr_utf8 () }
};
}
