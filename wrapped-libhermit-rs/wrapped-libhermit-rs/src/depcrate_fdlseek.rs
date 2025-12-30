// Generated macro for lseek (function)
macro_rules! Depcrate_fdlseek {
() => {
// Module: crate::fd
// Provides: {"lseek"}
// Dependencies: {}
pub (crate) fn lseek (fd : FileDescriptor , offset : isize , whence : SeekWhence) -> io :: Result < isize > { let obj = get_object (fd) ? ; block_on (async { obj . read () . await . lseek (offset , whence) . await } , None) }
};
}
