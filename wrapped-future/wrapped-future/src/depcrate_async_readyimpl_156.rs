// Generated macro for impl_156 (impl)
macro_rules! Depcrate_async_readyimpl_156 {
() => {
// Module: crate::async_ready
// Provides: {"impl_156"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation_Impl < T > for ReadyOperation_Impl < T > { fn SetCompleted (& self , handler : Ref < AsyncOperationCompletedHandler < T > >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationCompletedHandler < T > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . result . clone () } }
};
}
