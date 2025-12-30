// Generated macro for Recorder (struct)
macro_rules! Depcrate_index_as_worktree_with_renames_recorderRecorder {
() => {
// Module: crate::index_as_worktree_with_renames::recorder
// Provides: {"Recorder"}
// Dependencies: {}
# [doc = " Convenience implementation of [`VisitEntry`] that collects all changes into a `Vec`."] # [derive (Debug , Default)] pub struct Recorder < 'index , T = () , U = () > { # [doc = " The collected changes."] pub records : Vec < Entry < 'index , T , U > > , }
};
}
