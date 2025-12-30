// Generated macro for impl_819 (impl)
macro_rules! Depcrate_util_poolimpl_819 {
() => {
// Module: crate::util::pool
// Provides: {"impl_819"}
// Dependencies: {}
impl < 'a , T : Send + core :: fmt :: Debug , F : Fn () -> T > core :: fmt :: Debug for PoolGuard < 'a , T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("PoolGuard") . field (& self . 0) . finish () } }
};
}
