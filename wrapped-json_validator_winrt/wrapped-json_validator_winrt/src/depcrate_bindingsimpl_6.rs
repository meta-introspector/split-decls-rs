// Generated macro for impl_6 (impl)
macro_rules! Depcrate_bindingsimpl_6 {
() => {
// Module: crate::bindings
// Provides: {"impl_6"}
// Dependencies: {}
impl IJsonValidator_Vtbl { pub const fn new < Identity : IJsonValidator_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Validate < Identity : IJsonValidator_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , value : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IJsonValidator_Impl :: Validate (this , core :: mem :: transmute (& value)) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IJsonValidator , OFFSET > () , Validate : Validate :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IJsonValidator as windows_core :: Interface > :: IID } }
};
}
