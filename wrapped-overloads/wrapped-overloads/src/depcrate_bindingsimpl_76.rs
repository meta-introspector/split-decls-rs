// Generated macro for impl_76 (impl)
macro_rules! Depcrate_bindingsimpl_76 {
() => {
// Module: crate::bindings
// Provides: {"impl_76"}
// Dependencies: {}
impl IE_Vtbl { pub const fn new < Identity : IE_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn MethodOne < Identity : IE_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IE_Impl :: MethodOne (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn MethodTwo < Identity : IE_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IE_Impl :: MethodTwo (this , a) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IE , OFFSET > () , MethodOne : MethodOne :: < Identity , OFFSET > , MethodTwo : MethodTwo :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IE as windows_core :: Interface > :: IID } }
};
}
