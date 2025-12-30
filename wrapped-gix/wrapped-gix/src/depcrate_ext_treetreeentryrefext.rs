// Generated macro for TreeEntryRefExt (trait)
macro_rules! Depcrate_ext_treeTreeEntryRefExt {
() => {
// Module: crate::ext::tree
// Provides: {"TreeEntryRefExt"}
// Dependencies: {}
# [doc = " Extensions for [EntryRef](gix_object::tree::EntryRef)."] pub trait TreeEntryRefExt < 'a > : 'a { # [doc = " Attach [`repo`](crate::Repository) to the given tree entry. It can be detached later with `detach()`."] fn attach < 'repo > (self , repo : & 'repo crate :: Repository) -> crate :: object :: tree :: EntryRef < 'repo , 'a > ; }
};
}
