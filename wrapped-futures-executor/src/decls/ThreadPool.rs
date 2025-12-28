macro_rules! deps {
    () => {
        PoolState!();
    };
}

macro_rules! ThreadPool {
    () => {
        deps!();
        # [doc = " A general-purpose thread pool for scheduling tasks that poll futures to"] # [doc = " completion."] # [doc = ""] # [doc = " The thread pool multiplexes any number of tasks onto a fixed number of"] # [doc = " worker threads."] # [doc = ""] # [doc = " This type is a clonable handle to the threadpool itself."] # [doc = " Cloning it will only create a new reference, not a new threadpool."] # [doc = ""] # [doc = " This type is only available when the `thread-pool` feature of this"] # [doc = " library is activated."] # [cfg_attr (docsrs , doc (cfg (feature = "thread-pool")))] pub struct ThreadPool { state : Arc < PoolState > , }
    };
}

ThreadPool!()