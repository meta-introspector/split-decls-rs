macro_rules! ThreadPoolBuilder {
    () => {
        # [doc = " Thread pool configuration object."] # [doc = ""] # [doc = " This type is only available when the `thread-pool` feature of this"] # [doc = " library is activated."] # [cfg_attr (docsrs , doc (cfg (feature = "thread-pool")))] pub struct ThreadPoolBuilder { pool_size : usize , stack_size : usize , name_prefix : Option < String > , after_start : Option < Arc < dyn Fn (usize) + Send + Sync > > , before_stop : Option < Arc < dyn Fn (usize) + Send + Sync > > , }
    };
}

ThreadPoolBuilder!();