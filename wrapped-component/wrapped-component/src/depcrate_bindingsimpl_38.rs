// Generated macro for impl_38 (impl)
macro_rules! Depcrate_bindingsimpl_38 {
() => {
// Module: crate::bindings
// Provides: {"impl_38"}
// Dependencies: {}
impl IThing_Vtbl { pub const fn new < Identity : IThing_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Method < Identity : IThing_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IThing_Impl :: Method (this) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IThing , OFFSET > () , Method : Method :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IThing as windows_core :: Interface > :: IID } }
};
}
