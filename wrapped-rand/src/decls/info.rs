macro_rules! info {
    () => {
        # [allow (unused)] macro_rules ! info { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: info ! ($ ($ x) *) }) }
    };
}

info!()