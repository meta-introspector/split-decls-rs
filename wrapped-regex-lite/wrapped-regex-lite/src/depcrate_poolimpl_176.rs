// Generated macro for impl_176 (impl)
macro_rules! Depcrate_poolimpl_176 {
() => {
// Module: crate::pool
// Provides: {"impl_176"}
// Dependencies: {}
impl < T : Send , F : Fn () -> T > Pool < T , F > { # [doc = " Get a value from the pool. This may block if another thread is also"] # [doc = " attempting to retrieve a value from the pool."] pub (crate) fn get (& self) -> PoolGuard < '_ , T , F > { let mut stack = self . stack . lock () . unwrap () ; let value = match stack . pop () { None => Box :: new ((self . create) ()) , Some (value) => value , } ; PoolGuard { pool : self , value : Some (value) } } # [doc = " Puts a value back into the pool. Callers don't need to call this."] # [doc = " Once the guard that's returned by 'get' is dropped, it is put back"] # [doc = " into the pool automatically."] fn put_value (& self , value : Box < T >) { let mut stack = self . stack . lock () . unwrap () ; stack . push (value) ; } }
};
}
