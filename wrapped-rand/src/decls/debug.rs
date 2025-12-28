macro_rules! debug {
    () => {
        # [allow (unused)] macro_rules ! debug { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: debug ! ($ ($ x) *) }) }
    };
}

debug!()