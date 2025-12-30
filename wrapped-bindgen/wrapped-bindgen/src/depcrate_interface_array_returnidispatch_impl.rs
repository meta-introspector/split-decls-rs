// Generated macro for IDispatch_Impl (trait)
macro_rules! Depcrate_interface_array_returnIDispatch_Impl {
() => {
// Module: crate::interface_array_return
// Provides: {"IDispatch_Impl"}
// Dependencies: {}
pub trait IDispatch_Impl : windows_core :: IUnknownImpl { fn GetTypeInfoCount (& self) -> windows_core :: Result < u32 > ; fn GetIDsOfNames (& self , riid : * const windows_core :: GUID , rgsznames : * const windows_core :: PCWSTR , cnames : u32 , lcid : u32 , rgdispid : * mut i32 ,) -> windows_core :: Result < () > ; }
};
}
