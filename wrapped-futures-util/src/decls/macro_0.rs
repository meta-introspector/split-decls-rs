macro_rules! macro_0 {
    () => {
        # [cfg (all (feature = "bilock" , not (feature = "unstable")))] compile_error ! ("The `bilock` feature requires the `unstable` feature as an explicit opt-in to unstable features") ;
    };
}

macro_0!()