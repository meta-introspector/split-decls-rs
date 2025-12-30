// Generated macro for impl_720 (impl)
macro_rules! Depcrate_struct_with_cpp_interfaceimpl_720 {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"impl_720"}
// Dependencies: {}
impl ID3D12Pageable_Vtbl { pub const fn new < Identity : ID3D12Pageable_Impl , const OFFSET : isize > () -> Self { Self { base__ : ID3D12DeviceChild_Vtbl :: new :: < Identity , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID3D12Pageable as windows_core :: Interface > :: IID || iid == & < ID3D12Object as windows_core :: Interface > :: IID || iid == & < ID3D12DeviceChild as windows_core :: Interface > :: IID } }
};
}
