// Generated macro for IAsyncInfo_Impl (trait)
macro_rules! Depcrate_interface_requiredIAsyncInfo_Impl {
() => {
// Module: crate::interface_required
// Provides: {"IAsyncInfo_Impl"}
// Dependencies: {}
pub trait IAsyncInfo_Impl : windows_core :: IUnknownImpl { fn Id (& self) -> windows_core :: Result < u32 > ; fn ErrorCode (& self) -> windows_core :: Result < windows_core :: HRESULT > ; fn Cancel (& self) -> windows_core :: Result < () > ; fn Close (& self) -> windows_core :: Result < () > ; }
};
}
