macro_rules! deps {
    () => {
        GUID!();
        RPC_STATUS!();
    };
}

macro_rules! macro_12 {
    () => {
        deps!();
        windows_link :: link ! ("rpcrt4.dll" "system" fn UuidCreate (uuid : * mut GUID) -> RPC_STATUS) ;
    };
}

macro_12!()