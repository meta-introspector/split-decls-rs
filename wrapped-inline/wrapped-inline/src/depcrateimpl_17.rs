// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Downloads { fn next (& mut self , worker_id : WorkerId) -> Option < Download > { match self . pending . pop_front () { Some (d) => { self . in_progress . insert (worker_id , DownloadInProgress { id : d . id , started_at : Instant :: now () , progress : 0.0 , } ,) ; Some (d) } None => None , } } }
};
}
