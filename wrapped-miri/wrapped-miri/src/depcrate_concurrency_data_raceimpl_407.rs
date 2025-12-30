// Generated macro for impl_407 (impl)
macro_rules! Depcrate_concurrency_data_raceimpl_407 {
() => {
// Module: crate::concurrency::data_race
// Provides: {"impl_407"}
// Dependencies: {}
impl GlobalDataRaceHandler { # [doc = " Select whether data race checking is disabled. This is solely an"] # [doc = " implementation detail of `allow_data_races_*` and must not be used anywhere else!"] fn set_ongoing_action_data_race_free (& self , enable : bool) { match self { GlobalDataRaceHandler :: None => { } GlobalDataRaceHandler :: Vclocks (data_race) => { let old = data_race . ongoing_action_data_race_free . replace (enable) ; assert_ne ! (old , enable , "cannot nest allow_data_races") ; } GlobalDataRaceHandler :: Genmc (genmc_ctx) => { genmc_ctx . set_ongoing_action_data_race_free (enable) ; } } } }
};
}
