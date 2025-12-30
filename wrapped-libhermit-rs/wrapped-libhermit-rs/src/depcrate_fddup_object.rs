// Generated macro for dup_object (function)
macro_rules! Depcrate_fddup_object {
() => {
// Module: crate::fd
// Provides: {"dup_object"}
// Dependencies: {}
pub (crate) fn dup_object (fd : FileDescriptor) -> io :: Result < FileDescriptor > { core_scheduler () . dup_object (fd) }
};
}
