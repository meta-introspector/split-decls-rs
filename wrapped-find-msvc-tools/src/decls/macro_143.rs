macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! macro_143 {
    () => {
        deps!();
        RIDL ! { # [uuid (0x26aab78c , 0x4a60 , 0x49d6 , 0xaf , 0x3b , 0x3c , 0x35 , 0xbc , 0x93 , 0x36 , 0x5d)] interface ISetupConfiguration2 (ISetupConfiguration2Vtbl) : ISetupConfiguration (ISetupConfigurationVtbl) { fn EnumAllInstances (ppEnumInstances : * mut * mut IEnumSetupInstances ,) -> HRESULT , } }
    };
}

macro_143!();