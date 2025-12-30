// Generated macro for command_context (module)
macro_rules! Depcrate_configcommand_context {
() => {
// Module: crate::config
// Provides: {"command_context"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "attributes")] pub mod command_context { use crate :: config ; # [doc = " The error produced when collecting all information relevant to spawned commands,"] # [doc = " obtained via [Repository::command_context()](crate::Repository::command_context())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Boolean (# [from] config :: boolean :: Error) , # [error (transparent)] ParseBool (# [from] gix_config :: value :: Error) , } }
};
}
