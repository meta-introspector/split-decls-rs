// Generated macro for parse_core_abbrev (function)
macro_rules! Depcrate_config_cache_utilparse_core_abbrev {
() => {
// Module: crate::config::cache::util
// Provides: {"parse_core_abbrev"}
// Dependencies: {}
pub (crate) fn parse_core_abbrev (config : & gix_config :: File < 'static > , object_hash : gix_hash :: Kind ,) -> Result < Option < usize > , Error > { Ok (config . string ("core.abbrev") . map (| abbrev | Core :: ABBREV . try_into_abbreviation (abbrev , object_hash)) . transpose () ? . flatten ()) }
};
}
