macro_rules! deps {
    () => {
        WAIT_EVENT!();
        HANDLE!();
    };
}

macro_rules! macro_108 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn WaitForSingleObject (hhandle : HANDLE , dwmilliseconds : u32) -> WAIT_EVENT) ;
    };
}

macro_108!()