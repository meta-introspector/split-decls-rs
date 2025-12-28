macro_rules! deps {
    () => {
        IIterable_Impl!();
        IIterable!();
        IIterable_Vtbl!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IIterable_Vtbl < T > { pub const fn new < Identity : IIterable_Impl < T > , const OFFSET : isize > () -> Self { unsafe extern "system" fn First < T : windows_core :: RuntimeType + 'static , Identity : IIterable_Impl < T > , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IIterable_Impl :: First (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IIterable < T > , OFFSET > () , First : First :: < T , Identity , OFFSET > , T : core :: marker :: PhantomData :: < T > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IIterable < T > as windows_core :: Interface > :: IID } }
    };
}

impl_8!()