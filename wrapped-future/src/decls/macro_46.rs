macro_rules! deps {
    () => {
        IAsyncAction_Vtbl!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (IAsyncAction , IAsyncAction_Vtbl , 0x5a648006_843a_4da9_865b_9d26e5dfad7b) ;
    };
}

macro_46!()