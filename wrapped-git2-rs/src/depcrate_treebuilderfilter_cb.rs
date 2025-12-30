// Generated macro for filter_cb (function)
macro_rules! Depcrate_treebuilderfilter_cb {
() => {
// Module: crate::treebuilder
// Provides: {"filter_cb"}
// Dependencies: {}
extern "C" fn filter_cb (entry : * const raw :: git_tree_entry , payload : * mut c_void) -> c_int { let ret = panic :: wrap (| | unsafe { if panic :: panicked () { true } else { let entry = tree :: entry_from_raw_const (entry) ; let payload = payload as * mut & mut FilterCb < '_ > ; (* payload) (& entry) } }) ; if ret == Some (false) { 1 } else { 0 } }
};
}
