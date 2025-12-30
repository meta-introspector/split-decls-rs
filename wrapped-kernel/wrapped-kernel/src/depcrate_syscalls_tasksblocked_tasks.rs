// Generated macro for BLOCKED_TASKS (static)
macro_rules! Depcrate_syscalls_tasksBLOCKED_TASKS {
() => {
// Module: crate::syscalls::tasks
// Provides: {"BLOCKED_TASKS"}
// Dependencies: {}
# [doc = " Mapping between blocked tasks and their TaskHandle"] static BLOCKED_TASKS : InterruptTicketMutex < BTreeMap < TaskId , TaskHandle > > = InterruptTicketMutex :: new (BTreeMap :: new ()) ;
};
}
