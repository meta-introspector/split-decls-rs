// Generated macro for impl_198 (impl)
macro_rules! Depcrate_sync_wait_groupimpl_198 {
() => {
// Module: crate::sync::wait_group
// Provides: {"impl_198"}
// Dependencies: {}
impl fmt :: Debug for WaitGroup { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let count : & usize = & self . inner . count . lock () . unwrap () ; f . debug_struct ("WaitGroup") . field ("count" , count) . finish () } }
};
}
