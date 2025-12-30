// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bindingsimpl_23 {
() => {
// Module: crate::bindings
// Provides: {"impl_23"}
// Dependencies: {}
impl ICompositor_Vtbl { pub const fn new < Identity : ICompositor_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn CreateSpriteVisual < Identity : ICompositor_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , brush : i32 , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match ICompositor_Impl :: CreateSpriteVisual (this , brush) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn CreateContainerVisual < Identity : ICompositor_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , children : i32 , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match ICompositor_Impl :: CreateContainerVisual (this , children) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , ICompositor , OFFSET > () , CreateSpriteVisual : CreateSpriteVisual :: < Identity , OFFSET > , CreateContainerVisual : CreateContainerVisual :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ICompositor as windows_core :: Interface > :: IID } }
};
}
