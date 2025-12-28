macro_rules! macro_101 {
    () => {
        # [cfg (all (feature = "send_guard" , feature = "deadlock_detection"))] compile_error ! ("the `send_guard` and `deadlock_detection` features cannot be used together") ;
    };
}

macro_101!()