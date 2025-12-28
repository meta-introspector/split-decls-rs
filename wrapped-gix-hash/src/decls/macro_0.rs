macro_rules! macro_0 {
    () => {
        # [cfg (not (feature = "sha1"))] compile_error ! ("Please set the `sha1` feature flag") ;
    };
}

macro_0!();