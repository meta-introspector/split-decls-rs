// Generated macro for BlockedTaskQueue (struct)
macro_rules! Depcrate_scheduler_taskBlockedTaskQueue {
() => {
// Module: crate::scheduler::task
// Provides: {"BlockedTaskQueue"}
// Dependencies: {}
pub (crate) struct BlockedTaskQueue { list : LinkedList < BlockedTask > , # [cfg (feature = "net")] network_wakeup_time : Option < u64 > , }
};
}
