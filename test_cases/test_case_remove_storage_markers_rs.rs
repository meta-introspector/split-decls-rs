// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/remove_storage_markers.rs
// Error: expected square brackets
// Problematic line: line 9


pub(super) struct RemoveStorageMarkers;

impl<'tcx> crate::MirPass<'tcx> for RemoveStorageMarkers {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() > 0 && !sess.emit_lifetime_markers()
    }
