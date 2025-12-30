// Generated macro for impl_168 (impl)
macro_rules! Depcrate_registryimpl_168 {
() => {
// Module: crate::registry
// Provides: {"impl_168"}
// Dependencies: {}
impl < F > ThreadSpawn for CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { private_impl ! { } # [inline] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { (self . 0) (thread) } }
};
}
