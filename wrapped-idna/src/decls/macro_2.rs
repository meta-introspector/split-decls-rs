macro_rules! macro_2 {
    () => {
        # [cfg (not (feature = "alloc"))] compile_error ! ("the `alloc` feature must be enabled") ;
    };
}

macro_2!();