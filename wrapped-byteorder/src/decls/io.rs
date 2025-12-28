macro_rules! io {
    () => {
        # [cfg (feature = "std")] mod io ;
    };
}

io!()