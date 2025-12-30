// Generated macro for ID3D12Object_Impl (trait)
macro_rules! Depcrate_struct_with_cpp_interfaceID3D12Object_Impl {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"ID3D12Object_Impl"}
// Dependencies: {}
pub trait ID3D12Object_Impl : windows_core :: IUnknownImpl { fn GetPrivateData (& self , guid : * const windows_core :: GUID , pdatasize : * mut u32 , pdata : * mut core :: ffi :: c_void ,) -> windows_core :: Result < () > ; fn SetPrivateData (& self , guid : * const windows_core :: GUID , datasize : u32 , pdata : * const core :: ffi :: c_void ,) -> windows_core :: Result < () > ; fn SetPrivateDataInterface (& self , guid : * const windows_core :: GUID , pdata : windows_core :: Ref < windows_core :: IUnknown > ,) -> windows_core :: Result < () > ; fn SetName (& self , name : & windows_core :: PCWSTR) -> windows_core :: Result < () > ; }
};
}
