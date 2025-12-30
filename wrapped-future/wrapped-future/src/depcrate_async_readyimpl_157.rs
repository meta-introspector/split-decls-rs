// Generated macro for impl_157 (impl)
macro_rules! Depcrate_async_readyimpl_157 {
() => {
// Module: crate::async_ready
// Provides: {"impl_157"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress_Impl < P > for ReadyActionWithProgress_Impl < P > { fn SetCompleted (& self , handler : Ref < AsyncActionWithProgressCompletedHandler < P > >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionWithProgressCompletedHandler < P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . result . clone () } fn SetProgress (& self , _ : Ref < AsyncActionProgressHandler < P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncActionProgressHandler < P > > { Err (Error :: empty ()) } }
};
}
