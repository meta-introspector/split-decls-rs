macro_rules! deps {
    () => {
        HRESULT!();
        PULONGLONG!();
        LPCOLESTR!();
    };
}

macro_rules! macro_145 {
    () => {
        deps!();
        RIDL ! { # [uuid (0x42b21b78 , 0x6192 , 0x463e , 0x87 , 0xbf , 0xd5 , 0x77 , 0x83 , 0x8f , 0x1d , 0x5c)] interface ISetupHelper (ISetupHelperVtbl) : IUnknown (IUnknownVtbl) { fn ParseVersion (pwszVersion : LPCOLESTR , pullVersion : PULONGLONG ,) -> HRESULT , fn ParseVersionRange (pwszVersionRange : LPCOLESTR , pullMinVersion : PULONGLONG , pullMaxVersion : PULONGLONG ,) -> HRESULT , } }
    };
}

macro_145!();