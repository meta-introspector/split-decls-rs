// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/jump_threading.rs
// Error: expected square brackets
// Problematic line: line 62

const MAX_COST: usize = 100;
const MAX_PLACES: usize = 100;

impl<'tcx> crate::MirPass<'tcx> for JumpThreading {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() >= 2
    }
