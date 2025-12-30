// Generated macro for impl_176 (impl)
macro_rules! Depcrate_tree_ref_iterimpl_176 {
() => {
// Module: crate::tree::ref_iter
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a > TreeRefIter < 'a > { # [doc = " Consume self and return all parsed entries."] pub fn entries (self) -> Result < Vec < EntryRef < 'a > > , crate :: decode :: Error > { self . collect () } # [doc = " Return the offset in bytes that our data advanced from `buf`, the original buffer"] # [doc = " to the beginning of the data of the tree."] # [doc = ""] # [doc = " Then the tree-iteration can be resumed at the entry that would otherwise be returned next."] pub fn offset_to_next_entry (& self , buf : & [u8]) -> usize { let before = (* buf) . as_ptr () ; let after = (* self . data) . as_ptr () ; debug_assert ! (before <= after , "`TreeRefIter::offset_to_next_entry(): {after:?} <= {before:?}) violated") ; (after as usize - before as usize) / std :: mem :: size_of :: < u8 > () } }
};
}
