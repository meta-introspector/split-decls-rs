macro_rules! macro_0 {
    () => {
        # [cfg (all (target_family = "wasm" , not (target_os = "wasi")))] compile_error ! ("This wasm target is unsupported by mio. If using Tokio, disable the net feature.") ;
    };
}

macro_0!()