// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/dump_mir.rs
// Error: expected square brackets
// Problematic line: line 12


pub(super) struct Marker(pub &'static str);

impl<'tcx> crate::MirPass<'tcx> for Marker {
    fn name(&self) -> &'static str {
        self.0
    }
