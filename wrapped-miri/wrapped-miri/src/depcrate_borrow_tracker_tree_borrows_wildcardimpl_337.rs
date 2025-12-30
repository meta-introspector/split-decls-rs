// Generated macro for impl_337 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardimpl_337 {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"impl_337"}
// Dependencies: {}
impl Tree { # [doc = " Marks the tag as exposed & updates the wildcard tracking data structure"] # [doc = " to represent its access level."] # [doc = " Also takes as an argument whether the tag is protected or not."] pub fn expose_tag (& mut self , tag : BorTag , protected : bool) { let id = self . tag_mapping . get (& tag) . unwrap () ; let node = self . nodes . get_mut (id) . unwrap () ; node . is_exposed = true ; let node = self . nodes . get (id) . unwrap () ; for (_ , loc) in self . locations . iter_mut_all () { let perm = loc . perms . get (id) . map (| p | p . permission ()) . unwrap_or_else (| | node . default_location_state () . permission ()) ; let access_type = perm . strongest_allowed_child_access (protected) ; WildcardState :: update_exposure (id , access_type , & self . nodes , & mut loc . wildcard_accesses ,) ; } } # [doc = " This updates the wildcard tracking data structure to reflect the release of"] # [doc = " the protector on `tag`."] pub (super) fn update_exposure_for_protector_release (& mut self , tag : BorTag) { let idx = self . tag_mapping . get (& tag) . unwrap () ; if self . nodes . get (idx) . unwrap () . is_exposed { self . expose_tag (tag , false) ; } } }
};
}
