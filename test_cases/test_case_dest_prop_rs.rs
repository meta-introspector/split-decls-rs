// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/dest_prop.rs
// Error: expected square brackets
// Problematic line: line 154


pub(super) struct DestinationPropagation;

impl<'tcx> crate::MirPass<'tcx> for DestinationPropagation {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        // For now, only run at MIR opt level 3. Two things need to be changed before this can be
        // turned on by default:
