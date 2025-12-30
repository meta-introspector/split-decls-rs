// Generated macro for Parallelism (enum)
macro_rules! DepcrateParallelism {
() => {
// Module: crate
// Provides: {"Parallelism"}
// Dependencies: {}
# [doc = " Degree of parallelism to use when performing walk."] # [doc = ""] # [doc = " Parallelism happens at the directory level. It will help when walking deep"] # [doc = " filesystems with many directories. It wont help when reading a single"] # [doc = " directory with many files."] # [doc = ""] # [doc = " If you plan to perform lots of per file processing you might want to use Rayon to"] # [derive (Clone)] pub enum Parallelism { # [doc = " Run on calling thread, similar to what happens in the `walkdir` crate."] Serial , # [doc = " Run in default rayon thread pool."] RayonDefaultPool { # [doc = " Define when we consider the rayon default pool too busy to serve our iteration and abort the iteration, defaulting to 1s."] # [doc = ""] # [doc = " This can happen if `jwalk` is launched from within a par-iter on a pool that only has a single thread,"] # [doc = " or if there are many parallel `jwalk` invocations that all use the same threadpool, rendering it too busy"] # [doc = " to respond within this duration."] busy_timeout : std :: time :: Duration , } , # [doc = " Run in existing rayon thread pool"] RayonExistingPool { # [doc = " The pool to spawn our work onto."] pool : Arc < ThreadPool > , # [doc = " Similar to [`Parallelism::RayonDefaultPool::busy_timeout`] if `Some`, but can be `None` to skip the deadlock check"] # [doc = " in case you know that there is at least one free thread available on the pool."] busy_timeout : Option < std :: time :: Duration > , } , # [doc = " Run in new rayon thread pool with # threads"] RayonNewPool (usize) , }
};
}
