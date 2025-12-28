macro_rules! parse_derive {
    () => {
        # [cfg (not (feature = "export-internal"))] mod parse_derive ;
    };
}

parse_derive!();