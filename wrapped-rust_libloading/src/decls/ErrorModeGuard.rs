macro_rules! deps {
    () => {
        DWORD!();
    };
}

macro_rules! ErrorModeGuard {
    () => {
        deps!();
        struct ErrorModeGuard (DWORD) ;
    };
}

ErrorModeGuard!()