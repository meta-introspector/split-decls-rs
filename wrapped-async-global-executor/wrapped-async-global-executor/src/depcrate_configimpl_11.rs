// Generated macro for impl_11 (impl)
macro_rules! Depcrate_configimpl_11 {
() => {
// Module: crate::config
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Debug for GlobalExecutorConfig { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("GlobalExecutorConfig") . field ("env_var" , & self . env_var) . field ("min_threads" , & self . min_threads) . field ("max_threads" , & self . max_threads) . finish () } }
};
}
