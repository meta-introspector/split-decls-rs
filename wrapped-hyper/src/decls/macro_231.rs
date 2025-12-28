macro_rules! macro_231 {
    () => {
        # [cfg (not (all (feature = "client" , feature = "http1")))] compile_error ! ("The `ffi` feature currently requires the `client` and `http1` features.") ;
    };
}

macro_231!()