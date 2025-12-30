// Generated macro for add_locals_header_comment (function)
macro_rules! Depcrate_abi_commentsadd_locals_header_comment {
() => {
// Module: crate::abi::comments
// Provides: {"add_locals_header_comment"}
// Dependencies: {}
pub (super) fn add_locals_header_comment (fx : & mut FunctionCx < '_ , '_ , '_ >) { if fx . clif_comments . enabled () { fx . add_global_comment (String :: new ()) ; fx . add_global_comment ("kind  local ty                              size align (abi)" . to_string () ,) ; } }
};
}
