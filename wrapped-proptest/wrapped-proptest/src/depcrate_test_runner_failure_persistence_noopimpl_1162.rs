// Generated macro for impl_1162 (impl)
macro_rules! Depcrate_test_runner_failure_persistence_noopimpl_1162 {
() => {
// Module: crate::test_runner::failure_persistence::noop
// Provides: {"impl_1162"}
// Dependencies: {}
impl FailurePersistence for NoopFailurePersistence { fn load_persisted_failures2 (& self , _source_file : Option < & 'static str > ,) -> Vec < PersistedSeed > { Vec :: new () } fn save_persisted_failure2 (& mut self , _source_file : Option < & 'static str > , _seed : PersistedSeed , _shrunken_value : & dyn fmt :: Debug ,) { } fn box_clone (& self) -> Box < dyn FailurePersistence > { Box :: new (NoopFailurePersistence) } fn eq (& self , other : & dyn FailurePersistence) -> bool { other . as_any () . downcast_ref :: < Self > () . map_or (false , | x | x == self) } fn as_any (& self) -> & dyn Any { self } }
};
}
