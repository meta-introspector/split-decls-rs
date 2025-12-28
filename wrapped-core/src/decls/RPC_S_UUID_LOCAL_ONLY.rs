macro_rules! deps {
    () => {
        RPC_STATUS!();
    };
}

macro_rules! RPC_S_UUID_LOCAL_ONLY {
    () => {
        deps!();
        pub const RPC_S_UUID_LOCAL_ONLY : RPC_STATUS = 1824i32 ;
    };
}

RPC_S_UUID_LOCAL_ONLY!()