macro_rules! macro_0 {
    () => {
        # [cfg (all (feature = "rayon" , target_arch = "wasm32"))] compile_error ! ("Rayon cannot be used when targeting wasi32. Try disabling default features.") ;
    };
}

macro_0!()