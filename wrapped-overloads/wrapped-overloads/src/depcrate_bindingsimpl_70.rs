// Generated macro for impl_70 (impl)
macro_rules! Depcrate_bindingsimpl_70 {
() => {
// Module: crate::bindings
// Provides: {"impl_70"}
// Dependencies: {}
impl ID2_Vtbl { pub const fn new < Identity : ID2_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Method < Identity : ID2_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , b : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match ID2_Impl :: Method (this , a , b) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn Method2 < Identity : ID2_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , b : i32 , c : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match ID2_Impl :: Method2 (this , a , b , c) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , ID2 , OFFSET > () , Method : Method :: < Identity , OFFSET > , Method2 : Method2 :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID2 as windows_core :: Interface > :: IID } }
};
}
