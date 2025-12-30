// Generated macro for impl_923 (impl)
macro_rules! Depcrate_config_tree_sections_fetchimpl_923 {
() => {
// Module: crate::config::tree::sections::fetch
// Provides: {"impl_923"}
// Dependencies: {}
impl Fetch { # [doc = " The `fetch.negotiationAlgorithm` key."] pub const NEGOTIATION_ALGORITHM : NegotiationAlgorithm = NegotiationAlgorithm :: new_with_validate ("negotiationAlgorithm" , & config :: Tree :: FETCH , validate :: NegotiationAlgorithm ,) ; # [doc = " The `fetch.recurseSubmodules` key."] # [cfg (feature = "attributes")] pub const RECURSE_SUBMODULES : RecurseSubmodules = RecurseSubmodules :: new_with_validate ("recurseSubmodules" , & config :: Tree :: FETCH , validate :: RecurseSubmodules) ; }
};
}
