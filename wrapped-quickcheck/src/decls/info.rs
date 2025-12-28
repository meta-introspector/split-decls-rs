macro_rules! info {
    () => {
        # [cfg (not (feature = "use_logging"))] macro_rules ! info { ($ ($ _ignore : tt) *) => { () } ; }
    };
}

info!()