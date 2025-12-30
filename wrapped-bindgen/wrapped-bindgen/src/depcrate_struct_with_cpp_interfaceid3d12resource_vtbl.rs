// Generated macro for ID3D12Resource_Vtbl (struct)
macro_rules! Depcrate_struct_with_cpp_interfaceID3D12Resource_Vtbl {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"ID3D12Resource_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ID3D12Resource_Vtbl { pub base__ : ID3D12Pageable_Vtbl , Map : usize , Unmap : usize , GetDesc : usize , pub GetGPUVirtualAddress : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> u64 , WriteToSubresource : usize , ReadFromSubresource : usize , GetHeapProperties : usize , }
};
}
