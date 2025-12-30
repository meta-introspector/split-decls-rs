// Generated macro for impl_13 (impl)
macro_rules! Depcrate_persisting_hasherimpl_13 {
() => {
// Module: crate::persisting_hasher
// Provides: {"impl_13"}
// Dependencies: {}
impl PersistingHasherBuilder { pub fn flush (& self) { let mut guard = self . out . lock () . unwrap () ; guard . flush () . unwrap () ; } }
};
}
