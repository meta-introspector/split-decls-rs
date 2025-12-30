// Generated macro for ProgressId (enum)
macro_rules! Depcrate_fetch_typesProgressId {
() => {
// Module: crate::fetch::types
// Provides: {"ProgressId"}
// Dependencies: {}
# [doc = " The progress ids used in during various steps of the fetch operation."] # [doc = ""] # [doc = " Note that tagged progress isn't very widely available yet, but support can be improved as needed."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " The progress name is defined by the remote and the progress messages it sets, along with their progress values and limits."] RemoteProgress , }
};
}
