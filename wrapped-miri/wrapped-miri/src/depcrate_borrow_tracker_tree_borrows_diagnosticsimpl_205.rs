// Generated macro for impl_205 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_205 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_205"}
// Dependencies: {}
impl NodeDebugInfo { # [doc = " Information for a new node. By default it has no"] # [doc = " name and an empty history."] pub fn new (tag : BorTag , initial : Permission , span : Span) -> Self { let history = History { tag , created : (span , initial) , events : Vec :: new () } ; Self { tag , name : None , history } } # [doc = " Add a name to the tag. If a same tag is associated to several pointers,"] # [doc = " it can have several names which will be separated by commas."] pub fn add_name (& mut self , name : & str) { if let Some (prev_name) = & mut self . name { prev_name . push_str (", ") ; prev_name . push_str (name) ; } else { self . name = Some (String :: from (name)) ; } } }
};
}
