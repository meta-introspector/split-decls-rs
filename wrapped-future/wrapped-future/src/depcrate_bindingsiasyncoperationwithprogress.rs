// Generated macro for IAsyncOperationWithProgress (struct)
macro_rules! Depcrate_bindingsIAsyncOperationWithProgress {
() => {
// Module: crate::bindings
// Provides: {"IAsyncOperationWithProgress"}
// Dependencies: {}
# [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IAsyncOperationWithProgress < TResult , TProgress > (windows_core :: IUnknown , core :: marker :: PhantomData < TResult > , core :: marker :: PhantomData < TProgress > ,) where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static ;
};
}
