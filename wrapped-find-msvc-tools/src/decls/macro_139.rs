macro_rules! deps {
    () => {
        LCID!();
        LPFILETIME!();
        HRESULT!();
        LPCOLESTR!();
        BSTR!();
    };
}

macro_rules! macro_139 {
    () => {
        deps!();
        RIDL ! { # [uuid (0xb41463c3 , 0x8866 , 0x43b5 , 0xbc , 0x33 , 0x2b , 0x06 , 0x76 , 0xf7 , 0xf4 , 0x2e)] interface ISetupInstance (ISetupInstanceVtbl) : IUnknown (IUnknownVtbl) { fn GetInstanceId (pbstrInstanceId : * mut BSTR ,) -> HRESULT , fn GetInstallDate (pInstallDate : LPFILETIME ,) -> HRESULT , fn GetInstallationName (pbstrInstallationName : * mut BSTR ,) -> HRESULT , fn GetInstallationPath (pbstrInstallationPath : * mut BSTR ,) -> HRESULT , fn GetInstallationVersion (pbstrInstallationVersion : * mut BSTR ,) -> HRESULT , fn GetDisplayName (lcid : LCID , pbstrDisplayName : * mut BSTR ,) -> HRESULT , fn GetDescription (lcid : LCID , pbstrDescription : * mut BSTR ,) -> HRESULT , fn ResolvePath (pwszRelativePath : LPCOLESTR , pbstrAbsolutePath : * mut BSTR ,) -> HRESULT , } }
    };
}

macro_139!()