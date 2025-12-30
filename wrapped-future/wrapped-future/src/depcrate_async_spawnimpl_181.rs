// Generated macro for impl_181 (impl)
macro_rules! Depcrate_async_spawnimpl_181 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_181"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress_Impl < P > for ActionWithProgress_Impl < P > { fn SetCompleted (& self , handler : Ref < AsyncActionWithProgressCompletedHandler < P > >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionWithProgressCompletedHandler < P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . get_results () } fn SetProgress (& self , _ : Ref < AsyncActionProgressHandler < P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncActionProgressHandler < P > > { Err (Error :: empty ()) } }
};
}
