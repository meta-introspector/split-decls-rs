// Generated macro for impl_48 (impl)
macro_rules! Depcrate_comimpl_48 {
() => {
// Module: crate::com
// Provides: {"impl_48"}
// Dependencies: {}
impl ComPtr { pub fn as_raw (& self) -> * mut core :: ffi :: c_void { unsafe { core :: mem :: transmute_copy (self) } } pub fn cast (& self , iid : & GUID) -> Option < Self > { let mut result = None ; unsafe { com_call ! (IUnknown_Vtbl , self . QueryInterface (iid , & mut result as * mut _ as _)) ; } result } }
};
}
