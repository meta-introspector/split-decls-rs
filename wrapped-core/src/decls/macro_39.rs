macro_rules! deps {
    () => {
        IAgileObject_Vtbl!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IAgileObject , IAgileObject_Vtbl , 0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90) ;
    };
}

macro_39!()