// Generated macro for impl_25 (impl)
macro_rules! Depcrate_writerimpl_25 {
() => {
// Module: crate::writer
// Provides: {"impl_25"}
// Dependencies: {}
impl < D : Digest + Reset , W : io :: Write > Reset for HashWriter < D , W > { fn reset (& mut self) { Digest :: reset (& mut self . hasher) } }
};
}
