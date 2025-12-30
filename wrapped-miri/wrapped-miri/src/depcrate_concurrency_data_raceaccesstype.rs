// Generated macro for AccessType (enum)
macro_rules! Depcrate_concurrency_data_raceAccessType {
() => {
// Module: crate::concurrency::data_race
// Provides: {"AccessType"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Debug)] enum AccessType { NaRead (NaReadType) , NaWrite (NaWriteType) , AtomicLoad , AtomicStore , AtomicRmw , }
};
}
