// Generated macro for Execution (struct)
macro_rules! Depcrate_rt_executionExecution {
() => {
// Module: crate::rt::execution
// Provides: {"Execution"}
// Dependencies: {}
pub (crate) struct Execution { # [doc = " Uniquely identifies an execution"] pub (super) id : Id , # [doc = " Execution path taken"] pub (crate) path : Path , pub (crate) threads : thread :: Set , pub (crate) lazy_statics : lazy_static :: Set , # [doc = " All loom aware objects part of this execution run."] pub (super) objects : object :: Store , # [doc = " Maps raw allocations to LeakTrack objects"] pub (super) raw_allocations : HashMap < usize , Allocation > , pub (crate) arc_objs : HashMap < * const () , std :: sync :: Arc < super :: Arc > > , # [doc = " Maximum number of concurrent threads"] pub (super) max_threads : usize , pub (super) max_history : usize , # [doc = " Capture locations for significant events"] pub (crate) location : bool , # [doc = " Log execution output to STDOUT"] pub (crate) log : bool , }
};
}
