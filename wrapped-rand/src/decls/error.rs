macro_rules! error {
    () => {
        # [allow (unused)] macro_rules ! error { ($ ($ x : tt) *) => (# [cfg (feature = "log")] { log :: error ! ($ ($ x) *) }) }
    };
}

error!()