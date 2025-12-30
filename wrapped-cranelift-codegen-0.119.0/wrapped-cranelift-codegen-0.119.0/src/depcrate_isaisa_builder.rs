// Generated macro for isa_builder (macro)
macro_rules! Depcrate_isaisa_builder {
() => {
// Module: crate::isa
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Returns a builder that can create a corresponding `TargetIsa`"] # [doc = " or `Err(LookupError::SupportDisabled)` if not enabled."] macro_rules ! isa_builder { ($ name : ident , $ cfg_terms : tt , $ triple : ident) => { { # [cfg $ cfg_terms] { Ok ($ name :: isa_builder ($ triple)) } # [cfg (not $ cfg_terms)] { Err (LookupError :: SupportDisabled) } } } ; }
};
}
