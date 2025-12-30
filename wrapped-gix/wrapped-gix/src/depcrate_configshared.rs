// Generated macro for shared (module)
macro_rules! Depcrate_configshared {
() => {
// Module: crate::config
// Provides: {"shared"}
// Dependencies: {}
# [doc = " Utilities shared privately across the crate, for lack of a better place."] pub (crate) mod shared { use crate :: { config , config :: { cache :: util :: ApplyLeniency , tree :: Core } , } ; pub fn is_replace_refs_enabled (config : & gix_config :: File < 'static > , lenient : bool , mut filter_config_section : fn (& gix_config :: file :: Metadata) -> bool ,) -> Result < Option < bool > , config :: boolean :: Error > { config . boolean_filter ("core.useReplaceRefs" , & mut filter_config_section) . map (| b | Core :: USE_REPLACE_REFS . enrich_error (b)) . transpose () . with_leniency (lenient) } }
};
}
