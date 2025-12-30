// Generated macro for impl_29 (impl)
macro_rules! Depcrate_bindingsimpl_29 {
() => {
// Module: crate::bindings
// Provides: {"impl_29"}
// Dependencies: {}
impl IContainerVisual_Vtbl { pub const fn new < Identity : IContainerVisual_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Children < Identity : IContainerVisual_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; let ok__ = IContainerVisual_Impl :: Children (this) ; result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IContainerVisual , OFFSET > () , Children : Children :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IContainerVisual as windows_core :: Interface > :: IID } }
};
}
