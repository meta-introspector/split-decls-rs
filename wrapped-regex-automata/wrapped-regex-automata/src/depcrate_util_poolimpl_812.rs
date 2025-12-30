// Generated macro for impl_812 (impl)
macro_rules! Depcrate_util_poolimpl_812 {
() => {
// Module: crate::util::pool
// Provides: {"impl_812"}
// Dependencies: {}
impl < T , F > Pool < T , F > { # [doc = " Create a new pool. The given closure is used to create values in"] # [doc = " the pool when necessary."] pub fn new (create : F) -> Pool < T , F > { Pool (alloc :: boxed :: Box :: new (inner :: Pool :: new (create))) } }
};
}
