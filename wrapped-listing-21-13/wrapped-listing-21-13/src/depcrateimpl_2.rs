// Generated macro for impl_2 (impl)
macro_rules! Depcrateimpl_2 {
() => {
// Module: crate
// Provides: {"impl_2"}
// Dependencies: {}
impl ThreadPool { # [doc = " Create a new ThreadPool."] # [doc = ""] # [doc = " The size is the number of threads in the pool."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " The `new` function will panic if the size is zero."] pub fn new (size : usize) -> ThreadPool { assert ! (size > 0) ; ThreadPool } pub fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static , { } }
};
}
