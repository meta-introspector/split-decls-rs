// Generated macro for BlockContext (struct)
macro_rules! Depcrate_blockBlockContext {
() => {
// Module: crate::block
// Provides: {"BlockContext"}
// Dependencies: {}
# [doc = " A data structure holds contextual data for current block scope."] # [derive (Debug , Clone , Default)] pub struct BlockContext < 'rc > { # [doc = " the `base_path` of current block scope"] base_path : Vec < String > , # [doc = " the `base_value` of current block scope, when the block is using a"] # [doc = " constant or derived value as block base"] base_value : Option < Json > , # [doc = " current block context variables"] block_params : BlockParams < 'rc > , # [doc = " local variables in current context"] local_variables : LocalVars , }
};
}
