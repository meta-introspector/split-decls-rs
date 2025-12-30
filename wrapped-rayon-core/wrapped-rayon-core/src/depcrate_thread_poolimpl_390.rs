// Generated macro for impl_390 (impl)
macro_rules! Depcrate_thread_poolimpl_390 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_390"}
// Dependencies: {}
impl fmt :: Debug for ThreadPool { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ThreadPool") . field ("num_threads" , & self . current_num_threads ()) . field ("id" , & self . registry . id ()) . finish () } }
};
}
