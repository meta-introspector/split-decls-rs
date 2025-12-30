// Generated macro for impl_724 (impl)
macro_rules! Depcrate_cargoimpl_724 {
() => {
// Module: crate::cargo
// Provides: {"impl_724"}
// Dependencies: {}
impl Cargo { pub fn new (conf : & 'static Conf) -> Self { Self { allowed_duplicate_crates : conf . allowed_duplicate_crates . iter () . cloned () . collect () , ignore_publish : conf . cargo_ignore_publish , } } }
};
}
