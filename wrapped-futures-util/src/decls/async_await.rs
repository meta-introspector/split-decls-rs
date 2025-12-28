macro_rules! async_await {
    () => {
        # [cfg (feature = "async-await")] # [macro_use] mod async_await ;
    };
}

async_await!()