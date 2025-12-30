// Generated macro for PoolGuard (struct)
macro_rules! Depcrate_poolPoolGuard {
() => {
// Module: crate::pool
// Provides: {"PoolGuard"}
// Dependencies: {}
# [doc = " A guard that is returned when a caller requests a value from the pool."] pub (crate) struct PoolGuard < 'a , T : Send , F : Fn () -> T > { # [doc = " The pool that this guard is attached to."] pool : & 'a Pool < T , F > , # [doc = " This is None after the guard has been put back into the pool."] value : Option < Box < T > > , }
};
}
