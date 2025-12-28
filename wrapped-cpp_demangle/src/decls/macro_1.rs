macro_rules! macro_1 {
    () => {
        # [cfg (not (feature = "alloc"))] compile_error ! ("`alloc` or `std` feature is required for this crate") ;
    };
}

macro_1!();