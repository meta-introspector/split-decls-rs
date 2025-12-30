// Generated macro for checkout_options (module)
macro_rules! Depcrate_configcheckout_options {
() => {
// Module: crate::config
// Provides: {"checkout_options"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "attributes")] pub mod checkout_options { # [doc = " The error produced when collecting all information needed for checking out files into a worktree."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigCheckStat (# [from] super :: key :: GenericErrorWithValue) , # [error (transparent)] ConfigBoolean (# [from] super :: boolean :: Error) , # [error (transparent)] CheckoutWorkers (# [from] super :: checkout :: workers :: Error) , # [error (transparent)] Attributes (# [from] super :: attribute_stack :: Error) , # [error (transparent)] FilterPipelineOptions (# [from] crate :: filter :: pipeline :: options :: Error) , # [error (transparent)] CommandContext (# [from] crate :: config :: command_context :: Error) , } }
};
}
