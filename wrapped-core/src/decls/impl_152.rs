macro_rules! deps {
    () => {
        RuntimeName!();
        GUID!();
        IInspectable_Vtbl!();
        IUnknownImpl!();
        IUnknown_Vtbl!();
        HSTRING!();
        HRESULT!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl IInspectable_Vtbl { pub const fn new < Identity : IUnknownImpl , Name : RuntimeName , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetIids (_ : * mut c_void , count : * mut u32 , values : * mut * mut GUID ,) -> HRESULT { unsafe { if count . is_null () || values . is_null () { return imp :: E_POINTER ; } * count = 0 ; * values = null_mut () ; HRESULT (0) } } unsafe extern "system" fn GetRuntimeClassName < T : RuntimeName > (_ : * mut c_void , value : * mut * mut c_void ,) -> HRESULT { unsafe { if value . is_null () { return imp :: E_POINTER ; } # [cfg (windows)] { * value = core :: mem :: transmute :: < HSTRING , * mut c_void > (T :: NAME . into ()) ; } # [cfg (not (windows))] { * value = core :: ptr :: null_mut () ; } HRESULT (0) } } unsafe extern "system" fn GetTrustLevel < T : IUnknownImpl , const OFFSET : isize > (this : * mut c_void , value : * mut i32 ,) -> HRESULT { unsafe { if value . is_null () { return imp :: E_POINTER ; } let this = (this as * mut * mut c_void) . offset (OFFSET) as * mut T ; (* this) . GetTrustLevel (value) } } Self { base : IUnknown_Vtbl :: new :: < Identity , OFFSET > () , GetIids , GetRuntimeClassName : GetRuntimeClassName :: < Name > , GetTrustLevel : GetTrustLevel :: < Identity , OFFSET > , } } }
    };
}

impl_152!()