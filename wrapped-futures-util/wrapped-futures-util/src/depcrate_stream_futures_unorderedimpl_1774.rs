// Generated macro for impl_1774 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1774 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1774"}
// Dependencies: {}
impl Spawn for FuturesUnordered < FutureObj < '_ , () > > { fn spawn_obj (& self , future_obj : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . push (future_obj) ; Ok (()) } }
};
}
