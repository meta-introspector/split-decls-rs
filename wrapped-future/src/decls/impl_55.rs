macro_rules! deps {
    () => {
        IAsyncAction_Vtbl!();
        IAsyncAction_Impl!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl IAsyncAction_Vtbl { pub const fn new < Identity : IAsyncAction_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn SetCompleted < Identity : IAsyncAction_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , handler : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAsyncAction_Impl :: SetCompleted (this , core :: mem :: transmute_copy (& handler)) . into () } } unsafe extern "system" fn Completed < Identity : IAsyncAction_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IAsyncAction_Impl :: Completed (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn GetResults < Identity : IAsyncAction_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAsyncAction_Impl :: GetResults (this) . into () } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IAsyncAction , OFFSET > () , SetCompleted : SetCompleted :: < Identity , OFFSET > , Completed : Completed :: < Identity , OFFSET > , GetResults : GetResults :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAsyncAction as windows_core :: Interface > :: IID } }
    };
}

impl_55!();