macro_rules! deps {
    () => {
        DWORD!();
    };
}

macro_rules! SEM_FAILCRITICALERRORS {
    () => {
        deps!();
        const SEM_FAILCRITICALERRORS : DWORD = 1 ;
    };
}

SEM_FAILCRITICALERRORS!();