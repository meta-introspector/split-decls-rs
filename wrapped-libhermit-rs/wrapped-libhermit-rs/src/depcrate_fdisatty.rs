// Generated macro for isatty (function)
macro_rules! Depcrate_fdisatty {
() => {
// Module: crate::fd
// Provides: {"isatty"}
// Dependencies: {}
pub (crate) fn isatty (fd : FileDescriptor) -> io :: Result < bool > { let obj = get_object (fd) ? ; block_on (async { obj . read () . await . isatty () . await } , None) }
};
}
