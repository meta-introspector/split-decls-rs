// Generated macro for PerCoreSchedulerExt (trait)
macro_rules! Depcrate_schedulerPerCoreSchedulerExt {
() => {
// Module: crate::scheduler
// Provides: {"PerCoreSchedulerExt"}
// Dependencies: {}
pub (crate) trait PerCoreSchedulerExt { # [doc = " Triggers the scheduler to reschedule the tasks."] # [doc = " Interrupt flag will be cleared during the reschedule"] fn reschedule (self) ; # [cfg (feature = "net")] fn add_network_timer (self , wakeup_time : Option < u64 >) ; # [doc = " Terminate the current task on the current core."] fn exit (self , exit_code : i32) -> ! ; }
};
}
