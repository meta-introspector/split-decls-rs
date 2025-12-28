macro_rules! deps {
    () => {
        HRESULT!();
        PCWSTR!();
        BOOL!();
    };
}

macro_rules! macro_7 {
    () => {
        deps!();
        windows_link :: link ! ("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW (error : HRESULT , cchmax : u32 , message : PCWSTR) -> BOOL) ;
    };
}

macro_7!();