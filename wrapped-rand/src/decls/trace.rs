macro_rules! trace {
    () => {
        # [allow (unused)] macro_rules ! trace { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: trace ! ($ ($ x) *) }) }
    };
}

trace!();