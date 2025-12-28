macro_rules! warn {
    () => {
        # [allow (unused)] macro_rules ! warn { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: warn ! ($ ($ x) *) }) }
    };
}

warn!()