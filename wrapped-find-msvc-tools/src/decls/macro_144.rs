macro_rules! deps {
    () => {
        HRESULT!();
        BSTR!();
    };
}

macro_rules! macro_144 {
    () => {
        deps!();
        RIDL ! { # [uuid (0xda8d8a16 , 0xb2b6 , 0x4487 , 0xa2 , 0xf1 , 0x59 , 0x4c , 0xcc , 0xcd , 0x6b , 0xf5)] interface ISetupPackageReference (ISetupPackageReferenceVtbl) : IUnknown (IUnknownVtbl) { fn GetId (pbstrId : * mut BSTR ,) -> HRESULT , fn GetVersion (pbstrVersion : * mut BSTR ,) -> HRESULT , fn GetChip (pbstrChip : * mut BSTR ,) -> HRESULT , fn GetLanguage (pbstrLanguage : * mut BSTR ,) -> HRESULT , fn GetBranch (pbstrBranch : * mut BSTR ,) -> HRESULT , fn GetType (pbstrType : * mut BSTR ,) -> HRESULT , fn GetUniqueId (pbstrUniqueId : * mut BSTR ,) -> HRESULT , } }
    };
}

macro_144!()