// Generated macro for fstat (function)
macro_rules! Depcrate_fdfstat {
() => {
// Module: crate::fd
// Provides: {"fstat"}
// Dependencies: {}
pub fn fstat (fd : FileDescriptor) -> io :: Result < FileAttr > { let obj = get_object (fd) ? ; block_on (async { obj . read () . await . fstat () . await } , None) }
};
}
