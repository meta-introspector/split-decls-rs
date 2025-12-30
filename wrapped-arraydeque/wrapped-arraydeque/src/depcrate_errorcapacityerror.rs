// Generated macro for CapacityError (struct)
macro_rules! Depcrate_errorCapacityError {
() => {
// Module: crate::error
// Provides: {"CapacityError"}
// Dependencies: {}
# [doc = " Error value indicating insufficient capacity"] # [doc = ""] # [doc = " This error only occur to `ArrayDeque<_, Saturating>`."] # [derive (Clone , Copy , Eq , Ord , PartialEq , PartialOrd)] pub struct CapacityError < T = () > { # [doc = " The element that caused the error."] pub element : T , }
};
}
