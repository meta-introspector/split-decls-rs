// Generated macro for impl_46 (impl)
macro_rules! Depcrate_bindingsimpl_46 {
() => {
// Module: crate::bindings
// Provides: {"impl_46"}
// Dependencies: {}
impl IA_Vtbl { pub const fn new < Identity : IA_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Method < Identity : IA_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IA_Impl :: Method (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn Method2 < Identity : IA_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IA_Impl :: Method2 (this , a) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IA , OFFSET > () , Method : Method :: < Identity , OFFSET > , Method2 : Method2 :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IA as windows_core :: Interface > :: IID } }
};
}
