// Generated macro for IAsyncOperation_Impl (trait)
macro_rules! Depcrate_interface_genericIAsyncOperation_Impl {
() => {
// Module: crate::interface_generic
// Provides: {"IAsyncOperation_Impl"}
// Dependencies: {}
pub trait IAsyncOperation_Impl < TResult > : IAsyncInfo_Impl where TResult : windows_core :: RuntimeType + 'static , { fn GetResults (& self) -> windows_core :: Result < TResult > ; }
};
}
