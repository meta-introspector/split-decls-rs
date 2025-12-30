// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bindingsimpl_12 {
() => {
// Module: crate::bindings
// Provides: {"impl_12"}
// Dependencies: {}
impl IMissing_Vtbl { pub const fn new < Identity : IMissing_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Method < Identity : IMissing_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IMissing_Impl :: Method (this) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IMissing , OFFSET > () , Method : Method :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IMissing as windows_core :: Interface > :: IID } }
};
}
