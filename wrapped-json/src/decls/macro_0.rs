macro_rules! macro_0 {
    () => {
        # [cfg (not (any (feature = "std" , feature = "alloc")))] compile_error ! { "serde_json requires that either `std` (default) or `alloc` feature is enabled" }
    };
}

macro_0!()