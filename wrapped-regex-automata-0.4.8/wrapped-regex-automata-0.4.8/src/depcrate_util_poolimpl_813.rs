// Generated macro for impl_813 (impl)
macro_rules! Depcrate_util_poolimpl_813 {
() => {
// Module: crate::util::pool
// Provides: {"impl_813"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > PoolGuard < 'a , T , F > { # [doc = " Consumes this guard and puts it back into the pool."] # [doc = ""] # [doc = " This circumvents the guard's `Drop` implementation. This can be useful"] # [doc = " in circumstances where the automatic `Drop` results in poorer codegen,"] # [doc = " such as calling non-inlined functions."] # [inline] pub fn put (this : PoolGuard < '_ , T , F >) { inner :: PoolGuard :: put (this . 0) ; } }
};
}
