// Generated macro for impl_11 (impl)
macro_rules! Depcrate_bindingsimpl_11 {
() => {
// Module: crate::bindings
// Provides: {"impl_11"}
// Dependencies: {}
impl IAsyncAction_Vtbl { pub const fn new < Identity : IAsyncAction_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetResults < Identity : IAsyncAction_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAsyncAction_Impl :: GetResults (this) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IAsyncAction , OFFSET > () , SetCompleted : 0 , Completed : 0 , GetResults : GetResults :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAsyncAction as windows_core :: Interface > :: IID } }
};
}
