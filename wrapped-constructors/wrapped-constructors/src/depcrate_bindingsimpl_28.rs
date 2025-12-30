// Generated macro for impl_28 (impl)
macro_rules! Depcrate_bindingsimpl_28 {
() => {
// Module: crate::bindings
// Provides: {"impl_28"}
// Dependencies: {}
impl IActivatableFactory_Vtbl { pub const fn new < Identity : IActivatableFactory_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn WithValue < Identity : IActivatableFactory_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , arg : i32 , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IActivatableFactory_Impl :: WithValue (this , arg) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IActivatableFactory , OFFSET > () , WithValue : WithValue :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IActivatableFactory as windows_core :: Interface > :: IID } }
};
}
