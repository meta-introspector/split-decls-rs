// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bindingsimpl_12 {
() => {
// Module: crate::bindings
// Provides: {"impl_12"}
// Dependencies: {}
impl IJsonValidatorFactory_Vtbl { pub const fn new < Identity : IJsonValidatorFactory_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn CreateInstance < Identity : IJsonValidatorFactory_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , schema : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IJsonValidatorFactory_Impl :: CreateInstance (this , core :: mem :: transmute (& schema) ,) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IJsonValidatorFactory , OFFSET > () , CreateInstance : CreateInstance :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IJsonValidatorFactory as windows_core :: Interface > :: IID } }
};
}
