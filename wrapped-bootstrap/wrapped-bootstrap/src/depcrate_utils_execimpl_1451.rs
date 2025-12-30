// Generated macro for impl_1451 (impl)
macro_rules! Depcrate_utils_execimpl_1451 {
() => {
// Module: crate::utils::exec
// Provides: {"impl_1451"}
// Dependencies: {}
impl CommandCache { pub fn get (& self , key : & CommandFingerprint) -> Option < CommandOutput > { self . cache . lock () . unwrap () . get (key) . cloned () } pub fn insert (& self , key : CommandFingerprint , output : CommandOutput) { self . cache . lock () . unwrap () . insert (key , output) ; } }
};
}
