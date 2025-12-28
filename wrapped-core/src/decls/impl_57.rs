macro_rules! deps {
    () => {
        IWeakReference_Vtbl!();
        Interface!();
        IWeakReference_Impl!();
        HRESULT!();
        IUnknown_Vtbl!();
        GUID!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl IWeakReference_Vtbl { pub const fn new < Identity : IWeakReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Resolve < Identity : IWeakReference_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , riid : * const windows_core :: GUID , objectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IWeakReference_Impl :: Resolve (this , core :: mem :: transmute_copy (& riid) , core :: mem :: transmute_copy (& objectreference) ,) . into () } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , Resolve : Resolve :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IWeakReference as windows_core :: Interface > :: IID } }
    };
}

impl_57!()