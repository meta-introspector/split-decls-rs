// Generated macro for State (enum)
macro_rules! Depcrate_progressState {
() => {
// Module: crate::progress
// Provides: {"State"}
// Dependencies: {}
# [doc = " Indicate whether a progress can or cannot be made."] # [derive (Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug , Hash)] pub enum State { # [doc = " Indicates a task is blocked and cannot indicate progress, optionally until the"] # [doc = " given time. The task cannot easily be interrupted."] Blocked (& 'static str , Option < SystemTime >) , # [doc = " Indicates a task cannot indicate progress, optionally until the"] # [doc = " given time. The task can be interrupted."] Halted (& 'static str , Option < SystemTime >) , # [doc = " The task is running"] # [default] Running , }
};
}
