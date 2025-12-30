// Generated macro for impl_6 (impl)
macro_rules! Depcrate_bindingsimpl_6 {
() => {
// Module: crate::bindings
// Provides: {"impl_6"}
// Dependencies: {}
impl IInstance_Vtbl { pub const fn new < Identity : IInstance_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Property < Identity : IInstance_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut i32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IInstance_Impl :: Property (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IInstance , OFFSET > () , Property : Property :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IInstance as windows_core :: Interface > :: IID } }
};
}
