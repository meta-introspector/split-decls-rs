// Generated macro for treewalk_cb (function)
macro_rules! Depcrate_treetreewalk_cb {
() => {
// Module: crate::tree
// Provides: {"treewalk_cb"}
// Dependencies: {}
extern "C" fn treewalk_cb < T : Into < i32 > > (root : * const c_char , entry : * const raw :: git_tree_entry , payload : * mut c_void ,) -> c_int { match panic :: wrap (| | unsafe { let root = match CStr :: from_ptr (root) . to_str () { Ok (value) => value , _ => return - 1 , } ; let entry = entry_from_raw_const (entry) ; let payload = & mut * (payload as * mut TreeWalkCbData < '_ , T >) ; let callback = & mut payload . callback ; callback (root , & entry) . into () }) { Some (value) => value , None => - 1 , } }
};
}
