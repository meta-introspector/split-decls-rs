// Generated macro for impl_518 (impl)
macro_rules! Depcrate_syncimpl_518 {
() => {
// Module: crate::sync
// Provides: {"impl_518"}
// Dependencies: {}
impl < A > ActorContext for SyncContext < A > where A : Actor < Context = Self > , { # [doc = " Stop the current Actor. [`SyncContext`] will stop the existing Actor, and restart"] # [doc = " a new Actor of the same type to replace it."] fn stop (& mut self) { self . stopping = true ; self . state = ActorState :: Stopping ; } # [doc = " Terminate the current Actor. [`SyncContext`] will terminate the existing Actor, and restart"] # [doc = " a new Actor of the same type to replace it."] fn terminate (& mut self) { self . stopping = true ; self . state = ActorState :: Stopping ; } # [doc = " Get the Actor execution state."] fn state (& self) -> ActorState { self . state } }
};
}
