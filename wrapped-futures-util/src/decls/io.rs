macro_rules! io {
    () => {
        # [cfg (feature = "io")] # [cfg_attr (docsrs , doc (cfg (feature = "io")))] # [cfg (feature = "std")] pub mod io ;
    };
}

io!();