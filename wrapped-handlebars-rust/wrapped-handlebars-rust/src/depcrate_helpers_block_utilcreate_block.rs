// Generated macro for create_block (function)
macro_rules! Depcrate_helpers_block_utilcreate_block {
() => {
// Module: crate::helpers::block_util
// Provides: {"create_block"}
// Dependencies: {}
pub (crate) fn create_block < 'rc > (param : & PathAndJson < 'rc >) -> BlockContext < 'rc > { let mut block = BlockContext :: new () ; if let Some (new_path) = param . context_path () { block . base_path_mut () . clone_from (new_path) ; } else { block . set_base_value (param . value () . clone ()) ; } block }
};
}
