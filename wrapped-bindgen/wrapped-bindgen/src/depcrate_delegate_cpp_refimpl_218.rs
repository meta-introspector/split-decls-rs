// Generated macro for impl_218 (impl)
macro_rules! Depcrate_delegate_cpp_refimpl_218 {
() => {
// Module: crate::delegate_cpp_ref
// Provides: {"impl_218"}
// Dependencies: {}
impl IActivationFactory_Vtbl { pub const fn new < Identity : IActivationFactory_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn ActivateInstance < Identity : IActivationFactory_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , instance : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IActivationFactory_Impl :: ActivateInstance (this) { Ok (ok__) => { instance . write (core :: mem :: transmute (ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IActivationFactory , OFFSET > () , ActivateInstance : ActivateInstance :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IActivationFactory as windows_core :: Interface > :: IID } }
};
}
