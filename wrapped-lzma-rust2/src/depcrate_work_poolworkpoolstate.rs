// Generated macro for WorkPoolState (enum)
macro_rules! Depcrate_work_poolWorkPoolState {
() => {
// Module: crate::work_pool
// Provides: {"WorkPoolState"}
// Dependencies: {}
# [doc = " States for the work pool."] # [derive (Debug , Clone , Copy , PartialEq)] pub (crate) enum WorkPoolState { # [doc = " Actively accepting work and dispatching to threads."] Dispatching , # [doc = " No more work will be submitted, draining existing work."] Draining , # [doc = " All work completed."] Finished , # [doc = " An error occurred."] Error , }
};
}
