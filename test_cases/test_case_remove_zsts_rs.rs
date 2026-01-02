// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/remove_zsts.rs
// Error: expected square brackets
// Problematic line: line 9


pub(super) struct RemoveZsts;

impl<'tcx> crate::MirPass<'tcx> for RemoveZsts {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() > 0
    }
