macro_rules! deps {
    () => {
        AsyncActionCompletedHandler_Vtbl!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_core :: imp :: define_interface ! (AsyncActionCompletedHandler , AsyncActionCompletedHandler_Vtbl , 0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7) ;
    };
}

macro_1!()