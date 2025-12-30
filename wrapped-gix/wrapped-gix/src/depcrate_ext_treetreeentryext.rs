// Generated macro for TreeEntryExt (trait)
macro_rules! Depcrate_ext_treeTreeEntryExt {
() => {
// Module: crate::ext::tree
// Provides: {"TreeEntryExt"}
// Dependencies: {}
# [doc = " Extensions for [Entry](gix_object::tree::Entry)."] pub trait TreeEntryExt { # [doc = " Attach [`repo`](crate::Repository) to the given tree entry. It can be detached later with `detach()`."] fn attach (self , repo : & crate :: Repository) -> crate :: object :: tree :: Entry < '_ > ; }
};
}
