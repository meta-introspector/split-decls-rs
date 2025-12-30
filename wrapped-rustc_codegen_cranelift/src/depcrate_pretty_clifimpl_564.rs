// Generated macro for impl_564 (impl)
macro_rules! Depcrate_pretty_clifimpl_564 {
() => {
// Module: crate::pretty_clif
// Provides: {"impl_564"}
// Dependencies: {}
impl CommentWriter { pub (crate) fn new < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > ,) -> Self { let enabled = should_write_ir (tcx) ; let global_comments = if enabled { with_no_trimmed_paths ! ({ vec ! [format ! ("symbol {}" , tcx . symbol_name (instance) . name) , format ! ("instance {:?}" , instance) , format ! ("abi {:?}" , fn_abi) , String :: new () ,] }) } else { vec ! [] } ; CommentWriter { enabled , global_comments , entity_comments : FxHashMap :: default () , inst_post_comments : FxHashMap :: default () , } } }
};
}
