// Generated macro for impl_1522 (impl)
macro_rules! Depcrate_syscalls_condvarimpl_1522 {
() => {
// Module: crate::syscalls::condvar
// Provides: {"impl_1522"}
// Dependencies: {}
impl CondQueue { pub fn new () -> Self { CondQueue { counter : AtomicIsize :: new (0) , sem1 : Semaphore :: new (0) , sem2 : Semaphore :: new (0) , } } }
};
}
