// Generated macro for impl_182 (impl)
macro_rules! Depcrate_async_spawnimpl_182 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_182"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress_Impl < T , P > for OperationWithProgress_Impl < T , P > { fn SetCompleted (& self , handler : Ref < AsyncOperationWithProgressCompletedHandler < T , P > > ,) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationWithProgressCompletedHandler < T , P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . get_results () } fn SetProgress (& self , _ : Ref < AsyncOperationProgressHandler < T , P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncOperationProgressHandler < T , P > > { Err (Error :: empty ()) } }
};
}
