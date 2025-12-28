macro_rules! macro_0 {
    () => {
        # [cfg (feature = "cargo-all")] compile_error ! ("'--all-features' is not supported; use '--features all' instead") ;
    };
}

macro_0!()