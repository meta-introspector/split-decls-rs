// Generated macro for impl_61 (impl)
macro_rules! Depcrate_imp_com_bindingsimpl_61 {
() => {
// Module: crate::imp::com_bindings
// Provides: {"impl_61"}
// Dependencies: {}
impl IWeakReference_Vtbl { pub const fn new < Identity : IWeakReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Resolve < Identity : IWeakReference_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , riid : * const windows_core :: GUID , objectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IWeakReference_Impl :: Resolve (this , core :: mem :: transmute_copy (& riid) , core :: mem :: transmute_copy (& objectreference) ,) . into () } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , Resolve : Resolve :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IWeakReference as windows_core :: Interface > :: IID } }
};
}
