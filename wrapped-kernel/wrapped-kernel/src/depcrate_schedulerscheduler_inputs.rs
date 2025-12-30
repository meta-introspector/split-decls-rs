// Generated macro for SCHEDULER_INPUTS (static)
macro_rules! Depcrate_schedulerSCHEDULER_INPUTS {
() => {
// Module: crate::scheduler
// Provides: {"SCHEDULER_INPUTS"}
// Dependencies: {}
# [doc = " Map between Core ID and per-core scheduler"] # [cfg (feature = "smp")] static SCHEDULER_INPUTS : SpinMutex < Vec < & InterruptTicketMutex < SchedulerInput > > > = SpinMutex :: new (Vec :: new ()) ;
};
}
