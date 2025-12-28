macro_rules! AsyncExecutor {
    () => {
        # [doc = " Plugin trait used to allow benchmarking on multiple different async runtimes."] # [doc = ""] # [doc = " Smol, Tokio and Async-std are supported out of the box, as is the current-thread runner from the"] # [doc = " Futures crate; it is recommended to use whichever runtime you use in production."] pub trait AsyncExecutor { # [doc = " Spawn the given future onto this runtime and block until it's complete, returning the result."] fn block_on < T > (& self , future : impl Future < Output = T >) -> T ; }
    };
}

AsyncExecutor!();