// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/multiple_return_terminators.rs
// Error: expected square brackets
// Problematic line: line 12


pub(super) struct MultipleReturnTerminators;

impl<'tcx> crate::MirPass<'tcx> for MultipleReturnTerminators {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() >= 4
    }
