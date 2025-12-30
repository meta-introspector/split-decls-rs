// Generated macro for impl_47 (impl)
macro_rules! Depcrate_broadcastimpl_47 {
() => {
// Module: crate::broadcast
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > fmt :: Debug for BroadcastContext < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BroadcastContext") . field ("index" , & self . index ()) . field ("num_threads" , & self . num_threads ()) . field ("pool_id" , & self . worker . registry () . id ()) . finish () } }
};
}
