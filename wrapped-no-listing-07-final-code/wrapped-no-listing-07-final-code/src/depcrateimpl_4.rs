// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl ThreadPool { # [doc = " Create a new ThreadPool."] # [doc = ""] # [doc = " The size is the number of threads in the pool."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " The `new` function will panic if the size is zero."] pub fn new (size : usize) -> ThreadPool { assert ! (size > 0) ; let (sender , receiver) = mpsc :: channel () ; let receiver = Arc :: new (Mutex :: new (receiver)) ; let mut workers = Vec :: with_capacity (size) ; for id in 0 .. size { workers . push (Worker :: new (id , Arc :: clone (& receiver))) ; } ThreadPool { workers , sender : Some (sender) , } } pub fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static , { let job = Box :: new (f) ; self . sender . as_ref () . unwrap () . send (job) . unwrap () ; } }
};
}
