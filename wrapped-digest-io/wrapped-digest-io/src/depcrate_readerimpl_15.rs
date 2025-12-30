// Generated macro for impl_15 (impl)
macro_rules! Depcrate_readerimpl_15 {
() => {
// Module: crate::reader
// Provides: {"impl_15"}
// Dependencies: {}
impl < D : Digest + Reset , R : io :: Read > Reset for HashReader < D , R > { fn reset (& mut self) { Digest :: reset (& mut self . hasher) } }
};
}
