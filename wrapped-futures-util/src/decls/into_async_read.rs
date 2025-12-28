macro_rules! into_async_read {
    () => {
        # [cfg (feature = "io")] # [cfg (feature = "std")] mod into_async_read ;
    };
}

into_async_read!()