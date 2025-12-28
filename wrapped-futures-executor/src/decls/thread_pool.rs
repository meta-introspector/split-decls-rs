macro_rules! thread_pool {
    () => {
        # [cfg (feature = "thread-pool")] # [cfg_attr (docsrs , doc (cfg (feature = "thread-pool")))] # [cfg (feature = "std")] mod thread_pool ;
    };
}

thread_pool!()