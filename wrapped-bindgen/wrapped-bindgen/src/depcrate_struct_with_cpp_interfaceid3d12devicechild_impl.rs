// Generated macro for ID3D12DeviceChild_Impl (trait)
macro_rules! Depcrate_struct_with_cpp_interfaceID3D12DeviceChild_Impl {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"ID3D12DeviceChild_Impl"}
// Dependencies: {}
pub trait ID3D12DeviceChild_Impl : ID3D12Object_Impl { fn GetDevice (& self , riid : * const windows_core :: GUID , ppvdevice : * mut * mut core :: ffi :: c_void ,) -> windows_core :: Result < () > ; }
};
}
