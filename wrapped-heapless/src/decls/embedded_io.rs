macro_rules! embedded_io {
    () => {
        # [cfg (feature = "embedded-io-v0.7")] mod embedded_io ;
    };
}

embedded_io!();