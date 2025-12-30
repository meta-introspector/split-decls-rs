// Generated macro for impl_594 (impl)
macro_rules! Depcrate_reference_dependency_flatimpl_594 {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"impl_594"}
// Dependencies: {}
impl IClosable_Vtbl { pub const fn new < Identity : IClosable_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Close < Identity : IClosable_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IClosable_Impl :: Close (this) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IClosable , OFFSET > () , Close : Close :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IClosable as windows_core :: Interface > :: IID } }
};
}
