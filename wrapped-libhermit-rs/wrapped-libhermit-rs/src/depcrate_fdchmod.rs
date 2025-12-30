// Generated macro for chmod (function)
macro_rules! Depcrate_fdchmod {
() => {
// Module: crate::fd
// Provides: {"chmod"}
// Dependencies: {}
pub (crate) fn chmod (fd : FileDescriptor , mode : AccessPermission) -> io :: Result < () > { let obj = get_object (fd) ? ; block_on (async { obj . read () . await . chmod (mode) . await } , None) }
};
}
