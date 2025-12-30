// Generated macro for impl_182 (impl)
macro_rules! Depcrate_poolimpl_182 {
() => {
// Module: crate::pool
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a , T : Send + core :: fmt :: Debug , F : Fn () -> T > core :: fmt :: Debug for PoolGuard < 'a , T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("PoolGuard") . field ("pool" , & self . pool) . field ("value" , & self . value) . finish () } }
};
}
