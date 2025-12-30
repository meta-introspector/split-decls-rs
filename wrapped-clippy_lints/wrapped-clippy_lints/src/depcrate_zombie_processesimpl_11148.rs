// Generated macro for impl_11148 (impl)
macro_rules! Depcrate_zombie_processesimpl_11148 {
() => {
// Module: crate::zombie_processes
// Provides: {"impl_11148"}
// Dependencies: {}
impl Cause { fn message (self) -> & 'static str { match self { Cause :: NeverWait => "spawned process is never `wait()`ed on" , Cause :: EarlyReturn { .. } | Cause :: MissingWaitInBranch { .. } | Cause :: MissingElse { .. } => { "spawned process is not `wait()`ed on in all code paths" } , } } fn fallback_help (self) -> & 'static str { match self { Cause :: NeverWait => "consider calling `.wait()`" , Cause :: EarlyReturn { .. } | Cause :: MissingWaitInBranch { .. } | Cause :: MissingElse { .. } => { "consider calling `.wait()` in all code paths" } , } } }
};
}
