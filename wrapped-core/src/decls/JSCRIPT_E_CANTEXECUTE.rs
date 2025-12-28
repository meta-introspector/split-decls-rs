macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! JSCRIPT_E_CANTEXECUTE {
    () => {
        deps!();
        pub const JSCRIPT_E_CANTEXECUTE : windows_core :: HRESULT = windows_core :: HRESULT (0x89020001_u32 as _) ;
    };
}

JSCRIPT_E_CANTEXECUTE!();