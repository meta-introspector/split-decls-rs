// Generated macro for Recorder (struct)
macro_rules! Depcrate_index_as_worktree_recorderRecorder {
() => {
// Module: crate::index_as_worktree::recorder
// Provides: {"Recorder"}
// Dependencies: {}
# [doc = " Convenience implementation of [`VisitEntry`] that collects all non-trivial changes into a `Vec`."] # [derive (Debug , Default)] pub struct Recorder < 'index , T = () , U = () > { # [doc = " collected changes, index entries without conflicts or changes are excluded."] pub records : Vec < Record < 'index , T , U > > , }
};
}
