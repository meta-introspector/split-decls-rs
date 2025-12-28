macro_rules! macro_0 {
    () => {
        # [cfg (not (feature = "std"))] compile_error ! ("`futures-test` must have the `std` feature activated, this is a default-active feature") ;
    };
}

macro_0!()