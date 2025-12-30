// Generated macro for set_block_param (function)
macro_rules! Depcrate_helpers_helper_eachset_block_param {
() => {
// Module: crate::helpers::helper_each
// Provides: {"set_block_param"}
// Dependencies: {}
fn set_block_param < 'rc > (block : & mut BlockContext < 'rc > , h : & Helper < 'rc > , base_path : Option < & Vec < String > > , k : & Json , v : & Json ,) -> Result < () , RenderError > { if let Some (bp_val) = h . block_param () { let mut params = BlockParams :: new () ; if base_path . is_some () { params . add_path (bp_val , Vec :: with_capacity (0)) ? ; } else { params . add_value (bp_val , v . clone ()) ? ; } block . set_block_params (params) ; } else if let Some ((bp_val , bp_key)) = h . block_param_pair () { let mut params = BlockParams :: new () ; if base_path . is_some () { params . add_path (bp_val , Vec :: with_capacity (0)) ? ; } else { params . add_value (bp_val , v . clone ()) ? ; } params . add_value (bp_key , k . clone ()) ? ; block . set_block_params (params) ; } Ok (()) }
};
}
