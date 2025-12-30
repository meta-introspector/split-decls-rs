// Generated macro for IAsyncOperation_Impl (trait)
macro_rules! Depcrate_bindingsIAsyncOperation_Impl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncOperation_Impl"}
// Dependencies: {}
pub trait IAsyncOperation_Impl < TResult > : IAsyncInfo_Impl where TResult : windows_core :: RuntimeType + 'static , { fn SetCompleted (& self , handler : windows_core :: Ref < AsyncOperationCompletedHandler < TResult > > ,) -> windows_core :: Result < () > ; fn Completed (& self) -> windows_core :: Result < AsyncOperationCompletedHandler < TResult > > ; fn GetResults (& self) -> windows_core :: Result < TResult > ; }
};
}
