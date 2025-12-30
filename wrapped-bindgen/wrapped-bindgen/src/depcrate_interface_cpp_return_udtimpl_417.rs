// Generated macro for impl_417 (impl)
macro_rules! Depcrate_interface_cpp_return_udtimpl_417 {
() => {
// Module: crate::interface_cpp_return_udt
// Provides: {"impl_417"}
// Dependencies: {}
impl ID2D1Resource_Vtbl { pub const fn new < Identity : ID2D1Resource_Impl , const OFFSET : isize > () -> Self { Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , GetFactory : 0 , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID2D1Resource as windows_core :: Interface > :: IID } }
};
}
