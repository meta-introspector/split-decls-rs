// Generated macro for HomogeneousAggregate (enum)
macro_rules! Depcrate_callconvHomogeneousAggregate {
() => {
// Module: crate::callconv
// Provides: {"HomogeneousAggregate"}
// Dependencies: {}
# [doc = " Return value from the `homogeneous_aggregate` test function."] # [derive (Copy , Clone , Debug)] pub enum HomogeneousAggregate { # [doc = " Yes, all the \"leaf fields\" of this struct are passed in the"] # [doc = " same way (specified in the `Reg` value)."] Homogeneous (Reg) , # [doc = " There are no leaf fields at all."] NoData , }
};
}
