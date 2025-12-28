macro_rules! deps {
    () => {
        HRESULT!();
        ULONG!();
        REFIID!();
    };
}

macro_rules! macro_121 {
    () => {
        deps!();
        RIDL ! { # [uuid (0x00000000 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IUnknown (IUnknownVtbl) { fn QueryInterface (riid : REFIID , ppvObject : * mut * mut raw :: c_void ,) -> HRESULT , fn AddRef () -> ULONG , fn Release () -> ULONG , } }
    };
}

macro_121!();