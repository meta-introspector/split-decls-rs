macro_rules! io {
    () => {
        # [cfg (feature = "io-pipe")] pub mod io ;
    };
}

io!()