macro_rules! deps {
    () => {
        IAsyncOperation!();
        IAsyncOperation_Impl!();
        IAsyncOperation_Vtbl!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > IAsyncOperation_Vtbl < TResult > { pub const fn new < Identity : IAsyncOperation_Impl < TResult > , const OFFSET : isize > () -> Self { unsafe extern "system" fn SetCompleted < TResult : windows_core :: RuntimeType + 'static , Identity : IAsyncOperation_Impl < TResult > , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , handler : * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; IAsyncOperation_Impl :: SetCompleted (this , core :: mem :: transmute_copy (& handler)) . into () } } unsafe extern "system" fn Completed < TResult : windows_core :: RuntimeType + 'static , Identity : IAsyncOperation_Impl < TResult > , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IAsyncOperation_Impl :: Completed (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } unsafe extern "system" fn GetResults < TResult : windows_core :: RuntimeType + 'static , Identity : IAsyncOperation_Impl < TResult > , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , result__ : * mut windows_core :: AbiType < TResult > ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; match IAsyncOperation_Impl :: GetResults (this) { Ok (ok__) => { result__ . write (core :: mem :: transmute_copy (& ok__)) ; core :: mem :: forget (ok__) ; windows_core :: HRESULT (0) } Err (err) => err . into () , } } } Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IAsyncOperation < TResult > , OFFSET , > () , SetCompleted : SetCompleted :: < TResult , Identity , OFFSET > , Completed : Completed :: < TResult , Identity , OFFSET > , GetResults : GetResults :: < TResult , Identity , OFFSET > , TResult : core :: marker :: PhantomData :: < TResult > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAsyncOperation < TResult > as windows_core :: Interface > :: IID } }
    };
}

impl_89!();