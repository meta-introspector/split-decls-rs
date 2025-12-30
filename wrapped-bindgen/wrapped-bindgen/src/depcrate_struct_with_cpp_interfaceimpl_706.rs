// Generated macro for impl_706 (impl)
macro_rules! Depcrate_struct_with_cpp_interfaceimpl_706 {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"impl_706"}
// Dependencies: {}
impl ID3D12Object { pub unsafe fn GetPrivateData (& self , guid : * const windows_core :: GUID , pdatasize : * mut u32 , pdata : Option < * mut core :: ffi :: c_void > ,) -> windows_core :: Result < () > { unsafe { (windows_core :: Interface :: vtable (self) . GetPrivateData) (windows_core :: Interface :: as_raw (self) , guid , pdatasize as _ , pdata . unwrap_or (core :: mem :: zeroed ()) as _ ,) . ok () } } pub unsafe fn SetPrivateData (& self , guid : * const windows_core :: GUID , datasize : u32 , pdata : Option < * const core :: ffi :: c_void > ,) -> windows_core :: Result < () > { unsafe { (windows_core :: Interface :: vtable (self) . SetPrivateData) (windows_core :: Interface :: as_raw (self) , guid , datasize , pdata . unwrap_or (core :: mem :: zeroed ()) as _ ,) . ok () } } pub unsafe fn SetPrivateDataInterface < P1 > (& self , guid : * const windows_core :: GUID , pdata : P1 ,) -> windows_core :: Result < () > where P1 : windows_core :: Param < windows_core :: IUnknown > , { unsafe { (windows_core :: Interface :: vtable (self) . SetPrivateDataInterface) (windows_core :: Interface :: as_raw (self) , guid , pdata . param () . abi () ,) . ok () } } pub unsafe fn SetName < P0 > (& self , name : P0) -> windows_core :: Result < () > where P0 : windows_core :: Param < windows_core :: PCWSTR > , { unsafe { (windows_core :: Interface :: vtable (self) . SetName) (windows_core :: Interface :: as_raw (self) , name . param () . abi () ,) . ok () } } }
};
}
