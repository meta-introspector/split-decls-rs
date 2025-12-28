macro_rules! deps {
    () => {
        HANDLE!();
        WAIT_EVENT!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn WaitForSingleObject (hhandle : HANDLE , dwmilliseconds : u32) -> WAIT_EVENT) ;
    };
}

macro_38!();