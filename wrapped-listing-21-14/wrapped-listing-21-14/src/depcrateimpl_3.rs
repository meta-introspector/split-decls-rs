// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl ThreadPool { # [doc = " Create a new ThreadPool."] # [doc = ""] # [doc = " The size is the number of threads in the pool."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " The `new` function will panic if the size is zero."] pub fn new (size : usize) -> ThreadPool { assert ! (size > 0) ; let mut threads = Vec :: with_capacity (size) ; for _ in 0 .. size { } ThreadPool { threads } } pub fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static , { } }
};
}
