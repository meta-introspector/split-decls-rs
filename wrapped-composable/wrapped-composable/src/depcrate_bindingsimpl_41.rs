// Generated macro for impl_41 (impl)
macro_rules! Depcrate_bindingsimpl_41 {
() => {
// Module: crate::bindings
// Provides: {"impl_41"}
// Dependencies: {}
impl ISpriteVisual_Vtbl { pub const fn new < Identity : ISpriteVisual_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Brush < Identity : ISpriteVisual_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; let ok__ = ISpriteVisual_Impl :: Brush (this) ; result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , ISpriteVisual , OFFSET > () , Brush : Brush :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ISpriteVisual as windows_core :: Interface > :: IID } }
};
}
