macro_rules! blocking_io {
    () => {
        # [cfg (feature = "blocking-client")] mod blocking_io ;
    };
}

blocking_io!();