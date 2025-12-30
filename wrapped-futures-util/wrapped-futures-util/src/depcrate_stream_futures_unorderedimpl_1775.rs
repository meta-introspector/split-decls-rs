// Generated macro for impl_1775 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1775 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1775"}
// Dependencies: {}
impl LocalSpawn for FuturesUnordered < LocalFutureObj < '_ , () > > { fn spawn_local_obj (& self , future_obj : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { self . push (future_obj) ; Ok (()) } }
};
}
