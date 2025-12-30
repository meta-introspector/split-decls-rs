// Generated macro for impl_368 (impl)
macro_rules! Depcrate_providerimpl_368 {
() => {
// Module: crate::provider
// Provides: {"impl_368"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for WeekdaySet { fn bake (& self , ctx : & databake :: CrateEnv) -> databake :: TokenStream { ctx . insert ("icu_calendar") ; let days = crate :: week :: WeekdaySetIterator :: new (Weekday :: Monday , * self) . map (| d | d . bake (ctx)) ; databake :: quote ! { icu_calendar :: provider :: WeekdaySet :: new (& [# (# days) ,*]) } } }
};
}
