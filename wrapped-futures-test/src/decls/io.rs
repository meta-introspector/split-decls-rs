macro_rules! io {
    () => {
        # [cfg (feature = "std")] pub mod io ;
    };
}

io!();