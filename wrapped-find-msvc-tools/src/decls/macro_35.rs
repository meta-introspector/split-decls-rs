macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_35 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn ReleaseSemaphore (hsemaphore : HANDLE , lreleasecount : i32 , lppreviouscount : * mut i32) -> BOOL) ;
    };
}

macro_35!();