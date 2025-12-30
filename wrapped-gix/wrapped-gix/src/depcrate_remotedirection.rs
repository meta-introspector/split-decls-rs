// Generated macro for Direction (enum)
macro_rules! Depcrate_remoteDirection {
() => {
// Module: crate::remote
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " The direction of an operation carried out (or to be carried out) through a remote."] # [derive (Debug , Eq , PartialEq , Copy , Clone , Hash)] pub enum Direction { # [doc = " Push local changes to the remote."] Push , # [doc = " Fetch changes from the remote to the local repository."] Fetch , }
};
}
