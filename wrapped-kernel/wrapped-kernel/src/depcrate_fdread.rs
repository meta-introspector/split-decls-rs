// Generated macro for read (function)
macro_rules! Depcrate_fdread {
() => {
// Module: crate::fd
// Provides: {"read"}
// Dependencies: {}
pub (crate) fn read (fd : FileDescriptor , buf : & mut [u8]) -> io :: Result < usize > { let obj = get_object (fd) ? ; if buf . is_empty () { return Ok (0) ; } block_on (async { obj . read () . await . read (buf) . await } , None) }
};
}
