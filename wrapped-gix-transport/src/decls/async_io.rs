macro_rules! async_io {
    () => {
        # [doc = ""] # [cfg (feature = "async-client")] pub mod async_io ;
    };
}

async_io!()