// Generated macro for Note (enum)
macro_rules! Depcrate_config_tree_traitsNote {
() => {
// Module: crate::config::tree::traits
// Provides: {"Note"}
// Dependencies: {}
# [doc = " A note attached to a key."] # [derive (Debug , Copy , Clone)] pub enum Note { # [doc = " A piece of information related to a key to help the user."] Informative (& 'static str) , # [doc = " This key works differently than is described by git, explaining the deviation further."] Deviation (& 'static str) , }
};
}
