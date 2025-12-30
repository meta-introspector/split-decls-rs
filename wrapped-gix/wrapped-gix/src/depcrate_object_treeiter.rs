// Generated macro for iter (module)
macro_rules! Depcrate_object_treeiter {
() => {
// Module: crate::object::tree
// Provides: {"iter"}
// Dependencies: {}
# [doc = ""] mod iter { use super :: { EntryRef , Tree } ; impl < 'repo > Tree < 'repo > { # [doc = " Return an iterator over tree entries to obtain information about files and directories this tree contains."] pub fn iter (& self) -> impl Iterator < Item = Result < EntryRef < 'repo , '_ > , gix_object :: decode :: Error > > { let repo = self . repo ; gix_object :: TreeRefIter :: from_bytes (& self . data) . map (move | e | e . map (| entry | EntryRef { inner : entry , repo })) } } }
};
}
