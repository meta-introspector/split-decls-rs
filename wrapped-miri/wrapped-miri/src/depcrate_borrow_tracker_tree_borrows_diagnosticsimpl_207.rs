// Generated macro for impl_207 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_207 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_207"}
// Dependencies: {}
impl < 'tcx > Tree { # [doc = " Climb the tree to get the tag of a distant ancestor."] # [doc = " Allows operations on tags that are unreachable by the program"] # [doc = " but still exist in the tree. Not guaranteed to perform consistently"] # [doc = " if `provenance-gc=1`."] fn nth_parent (& self , tag : BorTag , nth_parent : u8) -> Option < BorTag > { let mut idx = self . tag_mapping . get (& tag) . unwrap () ; for _ in 0 .. nth_parent { let node = self . nodes . get (idx) . unwrap () ; idx = node . parent ? ; } Some (self . nodes . get (idx) . unwrap () . tag) } # [doc = " Debug helper: assign name to tag."] pub fn give_pointer_debug_name (& mut self , tag : BorTag , nth_parent : u8 , name : & str ,) -> InterpResult < 'tcx > { let tag = self . nth_parent (tag , nth_parent) . unwrap () ; let idx = self . tag_mapping . get (& tag) . unwrap () ; if let Some (node) = self . nodes . get_mut (idx) { node . debug_info . add_name (name) ; } else { eprintln ! ("Tag {tag:?} (to be named '{name}') not found!") ; } interp_ok (()) } # [doc = " Debug helper: determines if the tree contains a tag."] pub fn is_allocation_of (& self , tag : BorTag) -> bool { self . tag_mapping . contains_key (& tag) } }
};
}
