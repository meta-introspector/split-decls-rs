// Generated macro for CondQueue (struct)
macro_rules! Depcrate_syscalls_condvarCondQueue {
() => {
// Module: crate::syscalls::condvar
// Provides: {"CondQueue"}
// Dependencies: {}
struct CondQueue { counter : AtomicIsize , sem1 : Semaphore , sem2 : Semaphore , }
};
}
