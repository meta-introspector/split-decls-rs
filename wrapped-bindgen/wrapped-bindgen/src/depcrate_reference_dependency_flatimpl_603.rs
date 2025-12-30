// Generated macro for impl_603 (impl)
macro_rules! Depcrate_reference_dependency_flatimpl_603 {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"impl_603"}
// Dependencies: {}
impl IMemoryBufferReference_Vtbl { pub const fn new < Identity : IMemoryBufferReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Capacity < Identity : IMemoryBufferReference_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut u32 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IMemoryBufferReference_Impl :: Capacity (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn RemoveClosed < Identity : IMemoryBufferReference_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , cookie : i64 ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IMemoryBufferReference_Impl :: RemoveClosed (this , cookie) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IMemoryBufferReference , OFFSET > () , Capacity : Capacity :: < Identity , OFFSET > , Closed : 0 , RemoveClosed : RemoveClosed :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IMemoryBufferReference as windows_core :: Interface > :: IID } }
};
}
