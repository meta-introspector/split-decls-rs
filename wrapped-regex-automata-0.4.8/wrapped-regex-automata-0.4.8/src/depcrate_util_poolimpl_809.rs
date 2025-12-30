// Generated macro for impl_809 (impl)
macro_rules! Depcrate_util_poolimpl_809 {
() => {
// Module: crate::util::pool
// Provides: {"impl_809"}
// Dependencies: {}
impl < T , F > Pool < T , F > { # [doc = " Create a new pool. The given closure is used to create values in"] # [doc = " the pool when necessary."] pub fn new (create : F) -> Pool < T , F > { Pool (alloc :: boxed :: Box :: new (inner :: Pool :: new (create))) } }
};
}
