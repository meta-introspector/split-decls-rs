macro_rules! deps {
    () => {
        BSTR!();
        HRESULT!();
        InstanceState!();
        LPSAFEARRAY!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        RIDL ! { # [uuid (0x89143c9a , 0x05af , 0x49b0 , 0xb7 , 0x17 , 0x72 , 0xe2 , 0x18 , 0xa2 , 0x18 , 0x5c)] interface ISetupInstance2 (ISetupInstance2Vtbl) : ISetupInstance (ISetupInstanceVtbl) { fn GetState (pState : * mut InstanceState ,) -> HRESULT , fn GetPackages (ppsaPackages : * mut LPSAFEARRAY ,) -> HRESULT , fn GetProduct (ppPackage : * mut * mut ISetupPackageReference ,) -> HRESULT , fn GetProductPath (pbstrProductPath : * mut BSTR ,) -> HRESULT , } }
    };
}

macro_140!();