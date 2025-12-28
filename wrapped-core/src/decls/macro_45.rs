macro_rules! deps {
    () => {
        IAgileReference_Vtbl!();
    };
}

macro_rules! macro_45 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IAgileReference , IAgileReference_Vtbl , 0xc03f6a43_65a4_9818_987e_e0b810d2a6f2) ;
    };
}

macro_45!()