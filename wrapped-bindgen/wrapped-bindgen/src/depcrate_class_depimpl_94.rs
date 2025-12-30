// Generated macro for impl_94 (impl)
macro_rules! Depcrate_class_depimpl_94 {
() => {
// Module: crate::class_dep
// Provides: {"impl_94"}
// Dependencies: {}
impl IWwwFormUrlDecoderEntry_Vtbl { pub const fn new < Identity : IWwwFormUrlDecoderEntry_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Name < Identity : IWwwFormUrlDecoderEntry_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IWwwFormUrlDecoderEntry_Impl :: Name (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn Value < Identity : IWwwFormUrlDecoderEntry_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IWwwFormUrlDecoderEntry_Impl :: Value (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IWwwFormUrlDecoderEntry , OFFSET > () , Name : Name :: < Identity , OFFSET > , Value : Value :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IWwwFormUrlDecoderEntry as windows_core :: Interface > :: IID } }
};
}
