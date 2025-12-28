macro_rules! async_io {
    () => {
        # [cfg (feature = "async-client")] mod async_io ;
    };
}

async_io!();