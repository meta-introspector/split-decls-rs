// Generated macro for impl_2 (impl)
macro_rules! Depcrateimpl_2 {
() => {
// Module: crate
// Provides: {"impl_2"}
// Dependencies: {}
impl ThreadPool { pub fn new (size : usize) -> ThreadPool { ThreadPool } pub fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static , { } }
};
}
