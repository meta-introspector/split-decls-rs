macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
        WAIT_EVENT!();
    };
}

macro_rules! macro_93 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn WaitForSingleObjectEx (hhandle : HANDLE , dwmilliseconds : u32 , balertable : BOOL) -> WAIT_EVENT) ;
    };
}

macro_93!();