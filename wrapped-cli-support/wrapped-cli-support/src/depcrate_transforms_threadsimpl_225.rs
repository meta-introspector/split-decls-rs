// Generated macro for impl_225 (impl)
macro_rules! Depcrate_transforms_threadsimpl_225 {
() => {
// Module: crate::transforms::threads
// Provides: {"impl_225"}
// Dependencies: {}
impl ThreadCount { pub fn wrap_start (self , builder : & mut FunctionBuilder , start : FunctionId) { builder . func_body () . local_get (self . 0) . if_else (None , | _ | { } , | body | { body . call (start) ; } ,) ; } }
};
}
