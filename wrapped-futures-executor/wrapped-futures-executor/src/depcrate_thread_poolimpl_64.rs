// Generated macro for impl_64 (impl)
macro_rules! Depcrate_thread_poolimpl_64 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_64"}
// Dependencies: {}
impl Spawn for ThreadPool { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . spawn_obj_ok (future) ; Ok (()) } }
};
}
