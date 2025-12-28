macro_rules! deps {
    () => {
        IAsyncInfo_Vtbl!();
    };
}

macro_rules! macro_70 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IAsyncInfo , IAsyncInfo_Vtbl , 0x00000036_0000_0000_c000_000000000046) ;
    };
}

macro_70!()