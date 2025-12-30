// Generated macro for IAsyncOperationWithProgress_Impl (trait)
macro_rules! Depcrate_bindingsIAsyncOperationWithProgress_Impl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncOperationWithProgress_Impl"}
// Dependencies: {}
pub trait IAsyncOperationWithProgress_Impl < TResult , TProgress > : IAsyncInfo_Impl where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { fn SetProgress (& self , handler : windows_core :: Ref < AsyncOperationProgressHandler < TResult , TProgress > > ,) -> windows_core :: Result < () > ; fn Progress (& self) -> windows_core :: Result < AsyncOperationProgressHandler < TResult , TProgress > > ; fn SetCompleted (& self , handler : windows_core :: Ref < AsyncOperationWithProgressCompletedHandler < TResult , TProgress > > ,) -> windows_core :: Result < () > ; fn Completed (& self ,) -> windows_core :: Result < AsyncOperationWithProgressCompletedHandler < TResult , TProgress > > ; fn GetResults (& self) -> windows_core :: Result < TResult > ; }
};
}
