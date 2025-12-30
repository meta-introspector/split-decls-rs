// Generated macro for impl_440 (impl)
macro_rules! Depcrate_concurrency_init_onceimpl_440 {
() => {
// Module: crate::concurrency::init_once
// Provides: {"impl_440"}
// Dependencies: {}
impl InitOnce { # [inline] pub fn status (& self) -> InitOnceStatus { self . status } # [doc = " Begin initializing this InitOnce. Must only be called after checking that it is currently"] # [doc = " uninitialized."] # [inline] pub fn begin (& mut self) { assert_eq ! (self . status () , InitOnceStatus :: Uninitialized , "beginning already begun or complete init once") ; self . status = InitOnceStatus :: Begun ; } }
};
}
