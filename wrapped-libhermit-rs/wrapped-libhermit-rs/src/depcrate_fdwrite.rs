// Generated macro for write (function)
macro_rules! Depcrate_fdwrite {
() => {
// Module: crate::fd
// Provides: {"write"}
// Dependencies: {}
pub (crate) fn write (fd : FileDescriptor , buf : & [u8]) -> io :: Result < usize > { let obj = get_object (fd) ? ; if buf . is_empty () { return Ok (0) ; } block_on (async { obj . read () . await . write (buf) . await } , None) }
};
}
