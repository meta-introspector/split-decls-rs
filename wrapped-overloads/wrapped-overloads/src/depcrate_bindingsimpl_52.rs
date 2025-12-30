// Generated macro for impl_52 (impl)
macro_rules! Depcrate_bindingsimpl_52 {
() => {
// Module: crate::bindings
// Provides: {"impl_52"}
// Dependencies: {}
impl IB_Vtbl { pub const fn new < Identity : IB_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn MethodOne < Identity : IB_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IB_Impl :: MethodOne (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn MethodTwo < Identity : IB_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , a : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IB_Impl :: MethodTwo (this , a) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IB , OFFSET > () , MethodOne : MethodOne :: < Identity , OFFSET > , MethodTwo : MethodTwo :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IB as windows_core :: Interface > :: IID } }
};
}
