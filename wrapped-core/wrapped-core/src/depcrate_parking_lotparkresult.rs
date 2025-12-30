// Generated macro for ParkResult (enum)
macro_rules! Depcrate_parking_lotParkResult {
() => {
// Module: crate::parking_lot
// Provides: {"ParkResult"}
// Dependencies: {}
# [doc = " Result of a park operation."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ParkResult { # [doc = " We were unparked by another thread with the given token."] Unparked (UnparkToken) , # [doc = " The validation callback returned false."] Invalid , # [doc = " The timeout expired."] TimedOut , }
};
}
