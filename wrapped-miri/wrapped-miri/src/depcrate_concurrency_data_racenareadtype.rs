// Generated macro for NaReadType (enum)
macro_rules! Depcrate_concurrency_data_raceNaReadType {
() => {
// Module: crate::concurrency::data_race
// Provides: {"NaReadType"}
// Dependencies: {}
# [doc = " Type of a non-atomic read operation."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum NaReadType { # [doc = " Standard unsynchronized write."] Read , Retag , }
};
}
