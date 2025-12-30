// Generated macro for impl_140 (impl)
macro_rules! Depcrate_pipelineimpl_140 {
() => {
// Module: crate::pipeline
// Provides: {"impl_140"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Pipeline { # [doc = " Create a new pipeline with configured `drivers` (which should be considered safe to invoke), which are passed `context`."] # [doc = " `eol_config` serves as fallback to understand how to convert line endings if no line-ending attributes are present."] # [doc = " `crlf_roundtrip_check` corresponds to the git-configuration of `core.safecrlf`."] # [doc = " `object_hash` is relevant for the `ident` filter."] pub fn new (context : gix_command :: Context , options : Options) -> Self { let mut attrs = gix_attributes :: search :: Outcome :: default () ; attrs . initialize_with_selection (& Default :: default () , ATTRS) ; Pipeline { attrs , context : Context :: default () , processes : driver :: State :: new (context) , options , bufs : Default :: default () , } } # [doc = " Turn ourselves into state managing possibly running driver processes."] # [doc = ""] # [doc = " This can be used to control how these are terminated via [driver::State::shutdown()]."] pub fn into_driver_state (self) -> driver :: State { self . processes } }
};
}
