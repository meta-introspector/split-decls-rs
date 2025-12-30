// Generated macro for impl_435 (impl)
macro_rules! Depcrate_concurrency_data_raceimpl_435 {
() => {
// Module: crate::concurrency::data_race
// Provides: {"impl_435"}
// Dependencies: {}
impl NaWriteType { fn description (self) -> & 'static str { match self { NaWriteType :: Allocate => "creating a new allocation" , NaWriteType :: Write => "non-atomic write" , NaWriteType :: Retag => "retag write" , NaWriteType :: Deallocate => "deallocation" , } } }
};
}
