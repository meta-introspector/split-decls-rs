// Generated macro for truncate (function)
macro_rules! Depcrate_fdtruncate {
() => {
// Module: crate::fd
// Provides: {"truncate"}
// Dependencies: {}
pub (crate) fn truncate (fd : FileDescriptor , length : usize) -> io :: Result < () > { let obj = get_object (fd) ? ; block_on (async { obj . read () . await . truncate (length) . await } , None) }
};
}
