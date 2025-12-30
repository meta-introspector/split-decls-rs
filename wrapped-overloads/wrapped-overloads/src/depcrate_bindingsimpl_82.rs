// Generated macro for impl_82 (impl)
macro_rules! Depcrate_bindingsimpl_82 {
() => {
// Module: crate::bindings
// Provides: {"impl_82"}
// Dependencies: {}
impl IE2_Vtbl { pub const fn new < Identity : IE2_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn MethodThree < Identity : IE2_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , b : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IE2_Impl :: MethodThree (this , a , b) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn MethodFour < Identity : IE2_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , b : i32 , c : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IE2_Impl :: MethodFour (this , a , b , c) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IE2 , OFFSET > () , MethodThree : MethodThree :: < Identity , OFFSET > , MethodFour : MethodFour :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IE2 as windows_core :: Interface > :: IID } }
};
}
