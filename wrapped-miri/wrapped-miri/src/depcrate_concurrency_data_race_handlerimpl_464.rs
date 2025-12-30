// Generated macro for impl_464 (impl)
macro_rules! Depcrate_concurrency_data_race_handlerimpl_464 {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"impl_464"}
// Dependencies: {}
impl GlobalDataRaceHandler { pub fn is_none (& self) -> bool { matches ! (self , GlobalDataRaceHandler :: None) } pub fn as_vclocks_ref (& self) -> Option < & data_race :: GlobalState > { if let Self :: Vclocks (data_race) = self { Some (data_race) } else { None } } pub fn as_vclocks_mut (& mut self) -> Option < & mut data_race :: GlobalState > { if let Self :: Vclocks (data_race) = self { Some (data_race) } else { None } } pub fn as_genmc_ref (& self) -> Option < & GenmcCtx > { if let Self :: Genmc (genmc_ctx) = self { Some (genmc_ctx) } else { None } } }
};
}
