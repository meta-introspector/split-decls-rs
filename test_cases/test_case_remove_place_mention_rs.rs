// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/remove_place_mention.rs
// Error: expected square brackets
// Problematic line: line 9


pub(super) struct RemovePlaceMention;

impl<'tcx> crate::MirPass<'tcx> for RemovePlaceMention {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        !sess.opts.unstable_opts.mir_preserve_ub
    }
