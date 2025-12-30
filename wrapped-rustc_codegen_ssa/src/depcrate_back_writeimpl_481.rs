// Generated macro for impl_481 (impl)
macro_rules! Depcrate_back_writeimpl_481 {
() => {
// Module: crate::back::write
// Provides: {"impl_481"}
// Dependencies: {}
impl < B : ExtraBackendMethods > Coordinator < B > { fn join (mut self) -> std :: thread :: Result < Result < CompiledModules , () > > { self . future . take () . unwrap () . join () } }
};
}
