macro_rules! macro_0 {
    () => {
        # [cfg (not (feature = "std"))] compile_error ! ("`std` feature is currently required to build `clap`") ;
    };
}

macro_0!();