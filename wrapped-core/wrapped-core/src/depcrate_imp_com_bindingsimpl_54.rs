// Generated macro for impl_54 (impl)
macro_rules! Depcrate_imp_com_bindingsimpl_54 {
() => {
// Module: crate::imp::com_bindings
// Provides: {"impl_54"}
// Dependencies: {}
impl IAgileReference_Vtbl { pub const fn new < Identity : IAgileReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Resolve < Identity : IAgileReference_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , riid : * const windows_core :: GUID , ppvobjectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAgileReference_Impl :: Resolve (this , core :: mem :: transmute_copy (& riid) , core :: mem :: transmute_copy (& ppvobjectreference) ,) . into () } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , Resolve : Resolve :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAgileReference as windows_core :: Interface > :: IID } }
};
}
