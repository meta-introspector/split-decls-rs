macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! RPC_E_DISCONNECTED {
    () => {
        deps!();
        pub const RPC_E_DISCONNECTED : windows_core :: HRESULT = windows_core :: HRESULT (0x80010108_u32 as _) ;
    };
}

RPC_E_DISCONNECTED!()