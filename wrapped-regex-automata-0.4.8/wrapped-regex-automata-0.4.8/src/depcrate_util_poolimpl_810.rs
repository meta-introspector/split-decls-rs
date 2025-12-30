// Generated macro for impl_810 (impl)
macro_rules! Depcrate_util_poolimpl_810 {
() => {
// Module: crate::util::pool
// Provides: {"impl_810"}
// Dependencies: {}
impl < T : Send , F : Fn () -> T > Pool < T , F > { # [doc = " Get a value from the pool. The caller is guaranteed to have"] # [doc = " exclusive access to the given value. Namely, it is guaranteed that"] # [doc = " this will never return a value that was returned by another call to"] # [doc = " `get` but was not put back into the pool."] # [doc = ""] # [doc = " When the guard goes out of scope and its destructor is called, then"] # [doc = " it will automatically be put back into the pool. Alternatively,"] # [doc = " [`PoolGuard::put`] may be used to explicitly put it back in the pool"] # [doc = " without relying on its destructor."] # [doc = ""] # [doc = " Note that there is no guarantee provided about which value in the"] # [doc = " pool is returned. That is, calling get, dropping the guard (causing"] # [doc = " the value to go back into the pool) and then calling get again is"] # [doc = " *not* guaranteed to return the same value received in the first `get`"] # [doc = " call."] # [inline] pub fn get (& self) -> PoolGuard < '_ , T , F > { PoolGuard (self . 0 . get ()) } }
};
}
