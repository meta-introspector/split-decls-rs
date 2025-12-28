macro_rules! generator {
    () => {
        # [cfg (not (feature = "export-internal"))] mod generator ;
    };
}

generator!();