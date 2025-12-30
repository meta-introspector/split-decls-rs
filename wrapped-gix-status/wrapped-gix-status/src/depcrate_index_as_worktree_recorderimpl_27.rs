// Generated macro for impl_27 (impl)
macro_rules! Depcrate_index_as_worktree_recorderimpl_27 {
() => {
// Module: crate::index_as_worktree::recorder
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'index , T : Send , U : Send > VisitEntry < 'index > for Recorder < 'index , T , U > { type ContentChange = T ; type SubmoduleStatus = U ; fn visit_entry (& mut self , _entries : & 'index [index :: Entry] , entry : & 'index index :: Entry , entry_index : usize , relative_path : & 'index BStr , status : EntryStatus < Self :: ContentChange , Self :: SubmoduleStatus > ,) { self . records . push (Record { entry , entry_index , relative_path , status , }) ; } }
};
}
