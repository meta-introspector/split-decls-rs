// Generated macro for create (function)
macro_rules! Depcrate_tests_measurementcreate {
() => {
// Module: crate::tests::measurement
// Provides: {"create"}
// Dependencies: {}
# [test] fn create () { let mass = unsafe { NSMeasurement :: < NSUnitMass > :: new () } ; let _power = unsafe { mass . cast_unchecked :: < NSUnitPower > () } ; }
};
}
