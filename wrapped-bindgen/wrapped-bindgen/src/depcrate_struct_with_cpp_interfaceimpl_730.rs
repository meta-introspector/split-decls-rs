// Generated macro for impl_730 (impl)
macro_rules! Depcrate_struct_with_cpp_interfaceimpl_730 {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"impl_730"}
// Dependencies: {}
impl ID3D12Resource_Vtbl { pub const fn new < Identity : ID3D12Resource_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetGPUVirtualAddress < Identity : ID3D12Resource_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void ,) -> u64 { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; ID3D12Resource_Impl :: GetGPUVirtualAddress (this) } } Self { base__ : ID3D12Pageable_Vtbl :: new :: < Identity , OFFSET > () , Map : 0 , Unmap : 0 , GetDesc : 0 , GetGPUVirtualAddress : GetGPUVirtualAddress :: < Identity , OFFSET > , WriteToSubresource : 0 , ReadFromSubresource : 0 , GetHeapProperties : 0 , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID3D12Resource as windows_core :: Interface > :: IID || iid == & < ID3D12Object as windows_core :: Interface > :: IID || iid == & < ID3D12DeviceChild as windows_core :: Interface > :: IID || iid == & < ID3D12Pageable as windows_core :: Interface > :: IID } }
};
}
