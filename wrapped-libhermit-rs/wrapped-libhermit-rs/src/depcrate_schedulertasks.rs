// Generated macro for TASKS (static)
macro_rules! Depcrate_schedulerTASKS {
() => {
// Module: crate::scheduler
// Provides: {"TASKS"}
// Dependencies: {}
# [doc = " Map between Task ID and TaskHandle"] static TASKS : InterruptTicketMutex < BTreeMap < TaskId , TaskHandle > > = InterruptTicketMutex :: new (BTreeMap :: new ()) ;
};
}
