macro_rules! deps {
    () => {
        HANDLE!();
        SECURITY_ATTRIBUTES!();
        GUID!();
        PCWSTR!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        windows_link :: link ! ("ktmw32.dll" "system" fn CreateTransaction (lptransactionattributes : * mut SECURITY_ATTRIBUTES , uow : * mut GUID , createoptions : u32 , isolationlevel : u32 , isolationflags : u32 , timeout : u32 , description : PCWSTR) -> HANDLE) ;
    };
}

macro_3!();