// Generated macro for impl_351 (impl)
macro_rules! Depcrate_interfaceimpl_351 {
() => {
// Module: crate::interface
// Provides: {"impl_351"}
// Dependencies: {}
impl IStringable_Vtbl { pub const fn new < Identity : IStringable_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn ToString < Identity : IStringable_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IStringable_Impl :: ToString (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IStringable , OFFSET > () , ToString : ToString :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IStringable as windows_core :: Interface > :: IID } }
};
}
