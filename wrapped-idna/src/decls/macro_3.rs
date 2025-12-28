macro_rules! macro_3 {
    () => {
        # [cfg (not (feature = "compiled_data"))] compile_error ! ("the `compiled_data` feature must be enabled") ;
    };
}

macro_3!();