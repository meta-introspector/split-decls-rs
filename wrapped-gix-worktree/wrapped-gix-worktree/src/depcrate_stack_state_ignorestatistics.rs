// Generated macro for Statistics (struct)
macro_rules! Depcrate_stack_state_ignoreStatistics {
() => {
// Module: crate::stack::state::ignore
// Provides: {"Statistics"}
// Dependencies: {}
# [doc = " Various aggregate numbers related [`Ignore`]."] # [derive (Default , Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Statistics { # [doc = " Amount of patterns buffers read from the index."] pub patterns_buffers : usize , # [doc = " Amount of pattern files read from disk."] pub pattern_files : usize , # [doc = " Amount of pattern files we tried to find on disk."] pub tried_pattern_files : usize , }
};
}
