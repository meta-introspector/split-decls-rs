// Generated macro for impl_841 (impl)
macro_rules! Depcrate_providerimpl_841 {
() => {
// Module: crate::provider
// Provides: {"impl_841"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] impl icu_provider :: DataProvider < icu_time :: provider :: TimezonePeriodsV1 > for Baked { # [inline] fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_time :: provider :: TimezonePeriodsV1 > , icu_provider :: DataError , > { icu_time :: provider :: Baked . load (req) } }
};
}
