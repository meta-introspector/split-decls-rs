// Generated macro for impl_6 (impl)
macro_rules! Depcrate_bindingsimpl_6 {
() => {
// Module: crate::bindings
// Provides: {"impl_6"}
// Dependencies: {}
impl IReference_Vtbl { pub const fn new < Identity : IReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Method < Identity : IReference_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , stringable : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IReference_Impl :: Method (this , core :: mem :: transmute_copy (& stringable)) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IReference , OFFSET > () , Method : Method :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IReference as windows_core :: Interface > :: IID } }
};
}
