// Generated macro for add_args_header_comment (function)
macro_rules! Depcrate_abi_commentsadd_args_header_comment {
() => {
// Module: crate::abi::comments
// Provides: {"add_args_header_comment"}
// Dependencies: {}
pub (super) fn add_args_header_comment (fx : & mut FunctionCx < '_ , '_ , '_ >) { if fx . clif_comments . enabled () { fx . add_global_comment ("kind  loc.idx   param    pass mode                            ty" . to_string () ,) ; } }
};
}
