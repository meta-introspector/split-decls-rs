// Generated macro for impl_167 (impl)
macro_rules! Depcrate_registryimpl_167 {
() => {
// Module: crate::registry
// Provides: {"impl_167"}
// Dependencies: {}
impl < F > CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { pub (super) fn new (spawn : F) -> Self { CustomSpawn (spawn) } }
};
}
