macro_rules! blocking_io {
    () => {
        # [doc = ""] # [cfg (feature = "blocking-client")] pub mod blocking_io ;
    };
}

blocking_io!();