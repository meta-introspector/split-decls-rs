// Generated macro for Selected (enum)
macro_rules! Depcrate_selectSelected {
() => {
// Module: crate::select
// Provides: {"Selected"}
// Dependencies: {}
# [doc = " Current state of a select or a blocking operation."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Selected { # [doc = " Still waiting for an operation."] Waiting , # [doc = " The attempt to block the current thread has been aborted."] Aborted , # [doc = " An operation became ready because a channel is disconnected."] Disconnected , # [doc = " An operation became ready because a message can be sent or received."] Operation (Operation) , }
};
}
