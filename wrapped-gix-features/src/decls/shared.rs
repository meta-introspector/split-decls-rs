macro_rules! shared {
    () => {
        # [cfg (feature = "walkdir")] mod shared { # [doc = " The desired level of parallelism."] pub enum Parallelism { # [doc = " Do not parallelize at all by making a serial traversal on the current thread."] Serial , # [doc = " Create a new thread pool for each traversal with up to 16 threads or the amount of logical cores of the machine."] ThreadPoolPerTraversal { # [doc = " The base name of the threads we create as part of the thread-pool."] thread_name : & 'static str , } , } }
    };
}

shared!();