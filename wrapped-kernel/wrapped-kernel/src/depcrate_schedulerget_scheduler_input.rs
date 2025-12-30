// Generated macro for get_scheduler_input (function)
macro_rules! Depcrate_schedulerget_scheduler_input {
() => {
// Module: crate::scheduler
// Provides: {"get_scheduler_input"}
// Dependencies: {}
# [inline] # [cfg (feature = "smp")] fn get_scheduler_input (core_id : CoreId) -> & 'static InterruptTicketMutex < SchedulerInput > { SCHEDULER_INPUTS . lock () [usize :: try_from (core_id) . unwrap ()] }
};
}
