// Generated macro for disambiguate_hint (function)
macro_rules! Depcrate_config_cache_utildisambiguate_hint {
() => {
// Module: crate::config::cache::util
// Provides: {"disambiguate_hint"}
// Dependencies: {}
# [cfg (feature = "revision")] pub (crate) fn disambiguate_hint (config : & gix_config :: File < 'static > , lenient_config : bool ,) -> Result < Option < crate :: revision :: spec :: parse :: ObjectKindHint > , config :: key :: GenericErrorWithValue > { match config . string ("core.disambiguate") { None => Ok (None) , Some (value) => Core :: DISAMBIGUATE . try_into_object_kind_hint (value) . with_leniency (lenient_config) , } }
};
}
