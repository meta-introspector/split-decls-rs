macro_rules! deps {
    () => {
        IWeakReferenceSource_Vtbl!();
    };
}

macro_rules! macro_59 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IWeakReferenceSource , IWeakReferenceSource_Vtbl , 0x00000038_0000_0000_c000_000000000046) ;
    };
}

macro_59!();