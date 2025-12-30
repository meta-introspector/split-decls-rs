// Generated macro for impl_430 (impl)
macro_rules! Depcrate_concurrency_data_race_handlerimpl_430 {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"impl_430"}
// Dependencies: {}
impl VisitProvenance for AllocDataRaceHandler { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { AllocDataRaceHandler :: None => { } AllocDataRaceHandler :: Genmc => { } AllocDataRaceHandler :: Vclocks (data_race , weak_memory) => { data_race . visit_provenance (visit) ; weak_memory . visit_provenance (visit) ; } } } }
};
}
