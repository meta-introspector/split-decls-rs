// Generated macro for impl_180 (impl)
macro_rules! Depcrate_async_spawnimpl_180 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_180"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation_Impl < T > for Operation_Impl < T > { fn SetCompleted (& self , handler : Ref < AsyncOperationCompletedHandler < T > >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationCompletedHandler < T > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . get_results () } }
};
}
