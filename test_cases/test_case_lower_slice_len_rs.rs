// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/lower_slice_len.rs
// Error: expected square brackets
// Problematic line: line 10


pub(super) struct LowerSliceLenCalls;

impl<'tcx> crate::MirPass<'tcx> for LowerSliceLenCalls {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() > 0
    }
