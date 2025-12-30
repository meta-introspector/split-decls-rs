// Generated macro for IAsyncAction_Impl (trait)
macro_rules! Depcrate_bindingsIAsyncAction_Impl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncAction_Impl"}
// Dependencies: {}
pub trait IAsyncAction_Impl : IAsyncInfo_Impl { fn SetCompleted (& self , handler : windows_core :: Ref < AsyncActionCompletedHandler > ,) -> windows_core :: Result < () > ; fn Completed (& self) -> windows_core :: Result < AsyncActionCompletedHandler > ; fn GetResults (& self) -> windows_core :: Result < () > ; }
};
}
