// Generated macro for impl_1447 (impl)
macro_rules! Depcrate_status_iterimpl_1447 {
() => {
// Module: crate::status::iter
// Provides: {"impl_1447"}
// Dependencies: {}
impl Iter { fn maybe_keep_index_change (& mut self , item : Item) -> Option < Item > { match item { Item :: IndexWorktree (index_worktree :: Item :: Modification { status : EntryStatus :: NeedsUpdate (stat) , entry_index , .. }) => { self . index_changes . push ((entry_index , ApplyChange :: NewStat (stat))) ; return None ; } Item :: IndexWorktree (index_worktree :: Item :: Modification { status : EntryStatus :: Change (Change :: Modification { set_entry_stat_size_zero , .. }) , entry_index , .. }) if set_entry_stat_size_zero => { self . index_changes . push ((entry_index , ApplyChange :: SetSizeToZero)) ; } _ => { } } Some (item) } }
};
}
