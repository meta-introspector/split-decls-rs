// Generated macro for impl_86 (impl)
macro_rules! Depcrate_index_as_worktree_with_renames_recorderimpl_86 {
() => {
// Module: crate::index_as_worktree_with_renames::recorder
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'index , T : Send , U : Send > VisitEntry < 'index > for Recorder < 'index , T , U > { type ContentChange = T ; type SubmoduleStatus = U ; fn visit_entry (& mut self , entry : Entry < 'index , Self :: ContentChange , Self :: SubmoduleStatus >) { self . records . push (entry) ; } }
};
}
