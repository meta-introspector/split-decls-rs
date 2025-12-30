// Generated macro for merge_resource_cache (module)
macro_rules! Depcrate_repositorymerge_resource_cache {
() => {
// Module: crate::repository
// Provides: {"merge_resource_cache"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "merge")] pub mod merge_resource_cache { # [doc = " The error returned by [Repository::merge_resource_cache()](crate::Repository::merge_resource_cache())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] RenormalizeConfig (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] PipelineOptions (# [from] crate :: config :: merge :: pipeline_options :: Error) , # [error (transparent)] Index (# [from] crate :: repository :: index_or_load_from_head_or_empty :: Error) , # [error (transparent)] AttributeStack (# [from] crate :: config :: attribute_stack :: Error) , # [error (transparent)] CommandContext (# [from] crate :: config :: command_context :: Error) , # [error (transparent)] FilterPipeline (# [from] crate :: filter :: pipeline :: options :: Error) , # [error (transparent)] DriversConfig (# [from] crate :: config :: merge :: drivers :: Error) , } }
};
}
