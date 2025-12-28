macro_rules! macro_0 {
    () => {
        # [cfg (all (feature = "http3" , not (reqwest_unstable)))] compile_error ! ("\
    The `http3` feature is unstable, and requires the \
    `RUSTFLAGS='--cfg reqwest_unstable'` environment variable to be set.\
") ;
    };
}

macro_0!();