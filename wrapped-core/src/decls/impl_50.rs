macro_rules! deps {
    () => {
        GUID!();
        HRESULT!();
        IAgileReference_Impl!();
        IUnknown_Vtbl!();
        Interface!();
        IAgileReference_Vtbl!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl IAgileReference_Vtbl { pub const fn new < Identity : IAgileReference_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn Resolve < Identity : IAgileReference_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , riid : * const windows_core :: GUID , ppvobjectreference : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAgileReference_Impl :: Resolve (this , core :: mem :: transmute_copy (& riid) , core :: mem :: transmute_copy (& ppvobjectreference) ,) . into () } } Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , Resolve : Resolve :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAgileReference as windows_core :: Interface > :: IID } }
    };
}

impl_50!()