macro_rules! deps {
    () => {
        HRESULT!();
        CO_MTA_USAGE_COOKIE!();
    };
}

macro_rules! macro_4 {
    () => {
        deps!();
        windows_link :: link ! ("combase.dll" "system" fn CoIncrementMTAUsage (pcookie : * mut CO_MTA_USAGE_COOKIE) -> HRESULT) ;
    };
}

macro_4!();