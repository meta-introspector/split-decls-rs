macro_rules! deps {
    () => {
        AsyncActionCompletedHandlerBox!();
        AsyncStatus!();
        AsyncActionCompletedHandler_Vtbl!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < F : Fn (windows_core :: Ref < IAsyncAction > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > AsyncActionCompletedHandlerBox < F > { const VTABLE : AsyncActionCompletedHandler_Vtbl = AsyncActionCompletedHandler_Vtbl { base__ : windows_core :: IUnknown_Vtbl { QueryInterface : Self :: QueryInterface , AddRef : Self :: AddRef , Release : Self :: Release , } , Invoke : Self :: Invoke , } ; unsafe extern "system" fn QueryInterface (this : * mut core :: ffi :: c_void , iid : * const windows_core :: GUID , interface : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this = this as * mut * mut core :: ffi :: c_void as * mut Self ; if iid . is_null () || interface . is_null () { return windows_core :: HRESULT (- 2147467261) ; } * interface = if * iid == < AsyncActionCompletedHandler as windows_core :: Interface > :: IID || * iid == < windows_core :: IUnknown as windows_core :: Interface > :: IID || * iid == < windows_core :: imp :: IAgileObject as windows_core :: Interface > :: IID { & mut (* this) . vtable as * mut _ as _ } else if * iid == < windows_core :: imp :: IMarshal as windows_core :: Interface > :: IID { (* this) . count . add_ref () ; return windows_core :: imp :: marshaler (core :: mem :: transmute (& mut (* this) . vtable as * mut _ as * mut core :: ffi :: c_void) , interface ,) ; } else { core :: ptr :: null_mut () } ; if (* interface) . is_null () { windows_core :: HRESULT (- 2147467262) } else { (* this) . count . add_ref () ; windows_core :: HRESULT (0) } } } unsafe extern "system" fn AddRef (this : * mut core :: ffi :: c_void) -> u32 { unsafe { let this = this as * mut * mut core :: ffi :: c_void as * mut Self ; (* this) . count . add_ref () } } unsafe extern "system" fn Release (this : * mut core :: ffi :: c_void) -> u32 { unsafe { let this = this as * mut * mut core :: ffi :: c_void as * mut Self ; let remaining = (* this) . count . release () ; if remaining == 0 { let _ = windows_core :: imp :: Box :: from_raw (this) ; } remaining } } unsafe extern "system" fn Invoke (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT { unsafe { let this = & mut * (this as * mut * mut core :: ffi :: c_void as * mut Self) ; (this . invoke) (core :: mem :: transmute_copy (& asyncinfo) , asyncstatus) . into () } } }
    };
}

impl_6!();