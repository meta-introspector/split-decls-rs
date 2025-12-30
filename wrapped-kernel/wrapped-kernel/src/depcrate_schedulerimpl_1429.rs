// Generated macro for impl_1429 (impl)
macro_rules! Depcrate_schedulerimpl_1429 {
() => {
// Module: crate::scheduler
// Provides: {"impl_1429"}
// Dependencies: {}
# [cfg (feature = "smp")] impl SchedulerInput { pub fn new () -> Self { Self { new_tasks : VecDeque :: new () , wakeup_tasks : VecDeque :: new () , } } }
};
}
