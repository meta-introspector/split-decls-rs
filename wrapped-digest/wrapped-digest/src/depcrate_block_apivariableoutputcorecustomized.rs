// Generated macro for VariableOutputCoreCustomized (trait)
macro_rules! Depcrate_block_apiVariableOutputCoreCustomized {
() => {
// Module: crate::block_api
// Provides: {"VariableOutputCoreCustomized"}
// Dependencies: {}
# [doc = " Trait adding customization string to hash functions with variable output."] pub trait VariableOutputCoreCustomized : VariableOutputCore { # [doc = " Create new hasher instance with the given customization string and output size."] fn new_customized (customization : & [u8] , output_size : usize) -> Self ; }
};
}
