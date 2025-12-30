// Generated macro for impl_47 (impl)
macro_rules! Depcrate_bindingsimpl_47 {
() => {
// Module: crate::bindings
// Provides: {"impl_47"}
// Dependencies: {}
impl IVisual_Vtbl { pub const fn new < Identity : IVisual_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Compositor < Identity : IVisual_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IVisual_Impl :: Compositor (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IVisual , OFFSET > () , Compositor : Compositor :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IVisual as windows_core :: Interface > :: IID } }
};
}
