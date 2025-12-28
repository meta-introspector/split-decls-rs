macro_rules! deps {
    () => {
        IWeakReference_Vtbl!();
    };
}

macro_rules! macro_52 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IWeakReference , IWeakReference_Vtbl , 0x00000037_0000_0000_c000_000000000046) ;
    };
}

macro_52!();