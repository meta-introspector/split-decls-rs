// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bindingsimpl_22 {
() => {
// Module: crate::bindings
// Provides: {"impl_22"}
// Dependencies: {}
impl IActivatable_Vtbl { pub const fn new < Identity : IActivatable_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Property < Identity : IActivatable_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IActivatable_Impl :: Property (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IActivatable , OFFSET > () , Property : Property :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IActivatable as windows_core :: Interface > :: IID } }
};
}
