macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
    };
}

macro_rules! macro_2 {
    () => {
        deps!();
        windows_link :: link ! ("ktmw32.dll" "system" fn CommitTransaction (transactionhandle : HANDLE) -> BOOL) ;
    };
}

macro_2!()