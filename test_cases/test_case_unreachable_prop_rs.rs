// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/unreachable_prop.rs
// Error: expected square brackets
// Problematic line: line 16


pub(super) struct UnreachablePropagation;

impl crate::MirPass<'_> for UnreachablePropagation {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        // Enable only under -Zmir-opt-level=2 as this can make programs less debuggable.
        sess.mir_opt_level() >= 2
