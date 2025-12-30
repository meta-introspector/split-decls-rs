// Generated macro for impl_768 (impl)
macro_rules! Depcrate_struct_with_genericimpl_768 {
() => {
// Module: crate::struct_with_generic
// Provides: {"impl_768"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IReference_Vtbl < T > { pub const fn new < Identity : IReference_Impl < T > , const OFFSET : isize > () -> Self { unsafe extern "system" fn Value < T : windows_core :: RuntimeType + 'static , Identity : IReference_Impl < T > , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut windows_core :: AbiType < T > ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IReference_Impl :: Value (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IReference < T > , OFFSET > () , Value : Value :: < T , Identity , OFFSET > , T : core :: marker :: PhantomData :: < T > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IReference < T > as windows_core :: Interface > :: IID } }
};
}
