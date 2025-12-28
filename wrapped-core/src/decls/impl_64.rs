macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
        GUID!();
        IWeakReferenceSource_Impl!();
        IWeakReferenceSource_Vtbl!();
        Interface!();
        HRESULT!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl IWeakReferenceSource_Vtbl { pub const fn new < Identity : IWeakReferenceSource_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetWeakReference < Identity : IWeakReferenceSource_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , weakreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IWeakReferenceSource_Impl :: GetWeakReference (this) { Ok (ok__) => { weakreference . write (core :: mem :: transmute (ok__)) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , GetWeakReference : GetWeakReference :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IWeakReferenceSource as windows_core :: Interface > :: IID } }
    };
}

impl_64!();