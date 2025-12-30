// Generated macro for update_block_context (function)
macro_rules! Depcrate_helpers_helper_eachupdate_block_context {
() => {
// Module: crate::helpers::helper_each
// Provides: {"update_block_context"}
// Dependencies: {}
fn update_block_context (block : & mut BlockContext < '_ > , base_path : Option < & Vec < String > > , relative_path : String , is_first : bool , value : & Json ,) { if let Some (p) = base_path { if is_first { * block . base_path_mut () = copy_on_push_vec (p , relative_path) ; } else if let Some (ptr) = block . base_path_mut () . last_mut () { * ptr = relative_path ; } } else { block . set_base_value (value . clone ()) ; } }
};
}
