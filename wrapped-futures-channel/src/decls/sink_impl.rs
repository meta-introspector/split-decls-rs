macro_rules! sink_impl {
    () => {
        # [cfg (feature = "sink")] mod sink_impl ;
    };
}

sink_impl!();