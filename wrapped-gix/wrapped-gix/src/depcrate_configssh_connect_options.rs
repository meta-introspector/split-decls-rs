// Generated macro for ssh_connect_options (module)
macro_rules! Depcrate_configssh_connect_options {
() => {
// Module: crate::config
// Provides: {"ssh_connect_options"}
// Dependencies: {}
# [doc = ""] pub mod ssh_connect_options { # [doc = " The error produced when obtaining ssh connection configuration."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error (transparent)] pub struct Error (# [from] super :: key :: GenericErrorWithValue) ; }
};
}
