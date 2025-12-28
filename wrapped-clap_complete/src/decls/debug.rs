macro_rules! debug {
    () => {
        # [cfg (not (feature = "debug"))] macro_rules ! debug { ($ ($ arg : tt) *) => { } ; }
    };
}

debug!();