// Generated macro for LocationOutOfBoundsError (enum)
macro_rules! Depcrate_errorLocationOutOfBoundsError {
() => {
// Module: crate::error
// Provides: {"LocationOutOfBoundsError"}
// Dependencies: {}
# [doc = " A list of error outcomes for exceeding location bounds"] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [non_exhaustive] pub enum LocationOutOfBoundsError { # [doc = " Latitude value was out of bounds"] # [displaydoc ("Latitude {0} outside bounds of -90 to 90")] Latitude (f64) , # [doc = " Longitude value was out of bounds"] # [displaydoc ("Longitude {0} outside bounds of -180 to 180")] Longitude (f64) , # [doc = " Offset value was out of bounds"] # [displaydoc ("Offset {0} outside bounds of {1} to {2}")] Offset (f64 , f64 , f64) , }
};
}
