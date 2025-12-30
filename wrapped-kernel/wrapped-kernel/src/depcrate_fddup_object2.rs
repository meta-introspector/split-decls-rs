// Generated macro for dup_object2 (function)
macro_rules! Depcrate_fddup_object2 {
() => {
// Module: crate::fd
// Provides: {"dup_object2"}
// Dependencies: {}
pub (crate) fn dup_object2 (fd1 : FileDescriptor , fd2 : FileDescriptor) -> io :: Result < FileDescriptor > { core_scheduler () . dup_object2 (fd1 , fd2) }
};
}
