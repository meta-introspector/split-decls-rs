macro_rules! thread {
    () => {
        # [cfg (feature = "thread_rng")] pub (crate) mod thread ;
    };
}

thread!();