// Generated macro for ThreadPool (struct)
macro_rules! Depcrate_thread_poolThreadPool {
() => {
// Module: crate::thread_pool
// Provides: {"ThreadPool"}
// Dependencies: {}
# [doc = " Represents a user-created [thread pool]."] # [doc = ""] # [doc = " Use a [`ThreadPoolBuilder`] to specify the number and/or names of threads"] # [doc = " in the pool. After calling [`ThreadPoolBuilder::build()`], you can then"] # [doc = " execute functions explicitly within this [`ThreadPool`] using"] # [doc = " [`ThreadPool::install()`]. By contrast, top-level rayon functions"] # [doc = " (like `join()`) will execute implicitly within the current thread pool."] # [doc = ""] # [doc = ""] # [doc = " ## Creating a ThreadPool"] # [doc = ""] # [doc = " ```ignore-wasm"] # [doc = " # use rayon_core as rayon;"] # [doc = " let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " [`install()`][`ThreadPool::install()`] executes a closure in one of the `ThreadPool`'s"] # [doc = " threads. In addition, any other rayon operations called inside of `install()` will also"] # [doc = " execute in the context of the `ThreadPool`."] # [doc = ""] # [doc = " When the `ThreadPool` is dropped, that's a signal for the threads it manages to terminate,"] # [doc = " they will complete executing any remaining work that you have spawned, and automatically"] # [doc = " terminate."] # [doc = ""] # [doc = ""] # [doc = " [thread pool]: https://en.wikipedia.org/wiki/Thread_pool"] # [doc = " [`ThreadPoolBuilder::build()`]: ThreadPoolBuilder::build()"] # [doc = " [`ThreadPool::install()`]: Self::install()"] pub struct ThreadPool { registry : Arc < Registry > , }
};
}
