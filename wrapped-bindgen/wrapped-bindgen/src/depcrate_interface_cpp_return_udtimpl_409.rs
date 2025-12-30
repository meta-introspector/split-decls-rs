// Generated macro for impl_409 (impl)
macro_rules! Depcrate_interface_cpp_return_udtimpl_409 {
() => {
// Module: crate::interface_cpp_return_udt
// Provides: {"impl_409"}
// Dependencies: {}
impl ID2D1Image_Vtbl { pub const fn new < Identity : ID2D1Image_Impl , const OFFSET : isize > () -> Self { Self { base__ : ID2D1Resource_Vtbl :: new :: < Identity , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID2D1Image as windows_core :: Interface > :: IID || iid == & < ID2D1Resource as windows_core :: Interface > :: IID } }
};
}
