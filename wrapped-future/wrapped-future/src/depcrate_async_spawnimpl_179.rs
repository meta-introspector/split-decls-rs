// Generated macro for impl_179 (impl)
macro_rules! Depcrate_async_spawnimpl_179 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_179"}
// Dependencies: {}
impl IAsyncAction_Impl for Action_Impl { fn SetCompleted (& self , handler : Ref < AsyncActionCompletedHandler >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionCompletedHandler > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . get_results () } }
};
}
