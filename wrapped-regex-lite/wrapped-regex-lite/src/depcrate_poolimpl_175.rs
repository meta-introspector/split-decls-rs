// Generated macro for impl_175 (impl)
macro_rules! Depcrate_poolimpl_175 {
() => {
// Module: crate::pool
// Provides: {"impl_175"}
// Dependencies: {}
impl < T , F > Pool < T , F > { # [doc = " Create a new pool. The given closure is used to create values in"] # [doc = " the pool when necessary."] pub (crate) const fn new (create : F) -> Pool < T , F > { Pool { stack : Mutex :: new (vec ! []) , create } } }
};
}
