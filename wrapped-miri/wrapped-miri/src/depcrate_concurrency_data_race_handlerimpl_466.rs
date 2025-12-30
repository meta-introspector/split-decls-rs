// Generated macro for impl_466 (impl)
macro_rules! Depcrate_concurrency_data_race_handlerimpl_466 {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"impl_466"}
// Dependencies: {}
impl VisitProvenance for GlobalDataRaceHandler { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { GlobalDataRaceHandler :: None => { } GlobalDataRaceHandler :: Vclocks (data_race) => data_race . visit_provenance (visit) , GlobalDataRaceHandler :: Genmc (genmc_ctx) => genmc_ctx . visit_provenance (visit) , } } }
};
}
