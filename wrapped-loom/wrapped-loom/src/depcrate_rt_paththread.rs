// Generated macro for Thread (enum)
macro_rules! Depcrate_rt_pathThread {
() => {
// Module: crate::rt::path
// Provides: {"Thread"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , Clone , Copy)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) enum Thread { # [doc = " The thread is currently disabled"] Disabled , # [doc = " The thread should not be explored"] Skip , # [doc = " The thread is in a yield state."] Yield , # [doc = " The thread is waiting to be explored"] Pending , # [doc = " The thread is currently being explored"] Active , # [doc = " The thread has been explored"] Visited , }
};
}
