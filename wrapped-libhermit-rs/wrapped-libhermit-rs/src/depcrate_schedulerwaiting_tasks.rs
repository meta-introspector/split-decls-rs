// Generated macro for WAITING_TASKS (static)
macro_rules! Depcrate_schedulerWAITING_TASKS {
() => {
// Module: crate::scheduler
// Provides: {"WAITING_TASKS"}
// Dependencies: {}
# [doc = " Map between Task ID and Queue of waiting tasks"] static WAITING_TASKS : InterruptTicketMutex < BTreeMap < TaskId , VecDeque < TaskHandle > > > = InterruptTicketMutex :: new (BTreeMap :: new ()) ;
};
}
