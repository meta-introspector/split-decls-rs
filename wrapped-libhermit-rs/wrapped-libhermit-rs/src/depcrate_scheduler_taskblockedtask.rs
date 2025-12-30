// Generated macro for BlockedTask (struct)
macro_rules! Depcrate_scheduler_taskBlockedTask {
() => {
// Module: crate::scheduler::task
// Provides: {"BlockedTask"}
// Dependencies: {}
struct BlockedTask { task : Rc < RefCell < Task > > , wakeup_time : Option < u64 > , }
};
}
