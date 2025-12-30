// Generated macro for impl_155 (impl)
macro_rules! Depcrate_async_readyimpl_155 {
() => {
// Module: crate::async_ready
// Provides: {"impl_155"}
// Dependencies: {}
impl IAsyncAction_Impl for ReadyAction_Impl { fn SetCompleted (& self , handler : Ref < AsyncActionCompletedHandler >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionCompletedHandler > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . result . clone () } }
};
}
