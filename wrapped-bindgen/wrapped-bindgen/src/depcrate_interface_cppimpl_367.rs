// Generated macro for impl_367 (impl)
macro_rules! Depcrate_interface_cppimpl_367 {
() => {
// Module: crate::interface_cpp
// Provides: {"impl_367"}
// Dependencies: {}
impl IPersist_Vtbl { pub const fn new < Identity : IPersist_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetClassID < Identity : IPersist_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , pclassid : * mut windows_core :: GUID ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IPersist_Impl :: GetClassID (this) { Ok (ok__) => { pclassid . write (core :: mem :: transmute (ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , GetClassID : GetClassID :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IPersist as windows_core :: Interface > :: IID } }
};
}
