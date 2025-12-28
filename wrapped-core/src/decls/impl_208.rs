macro_rules! deps {
    () => {
        HRESULT!();
        IUnknown_Vtbl!();
        IUnknownImpl!();
        GUID!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl IUnknown_Vtbl { pub const fn new < T : IUnknownImpl , const OFFSET : isize > () -> Self { unsafe extern "system" fn QueryInterface < T : IUnknownImpl , const OFFSET : isize > (this : * mut c_void , iid : * const GUID , interface : * mut * mut c_void ,) -> HRESULT { unsafe { let this = (this as * mut * mut c_void) . offset (OFFSET) as * mut T ; (* this) . QueryInterface (iid , interface) } } unsafe extern "system" fn AddRef < T : IUnknownImpl , const OFFSET : isize > (this : * mut c_void ,) -> u32 { unsafe { let this = (this as * mut * mut c_void) . offset (OFFSET) as * mut T ; (* this) . AddRef () } } unsafe extern "system" fn Release < T : IUnknownImpl , const OFFSET : isize > (this : * mut c_void ,) -> u32 { unsafe { let this = (this as * mut * mut c_void) . offset (OFFSET) as * mut T ; T :: Release (this) } } Self { QueryInterface : QueryInterface :: < T , OFFSET > , AddRef : AddRef :: < T , OFFSET > , Release : Release :: < T , OFFSET > , } } }
    };
}

impl_208!()