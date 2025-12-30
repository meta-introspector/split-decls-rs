// Generated macro for TimeDiff (struct)
macro_rules! Depcrate_asn1TimeDiff {
() => {
// Module: crate::asn1
// Provides: {"TimeDiff"}
// Dependencies: {}
# [doc = " Difference between two ASN1 times."] # [doc = ""] # [doc = " This `struct` is created by the [`diff`] method on [`Asn1TimeRef`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`diff`]: struct.Asn1TimeRef.html#method.diff"] # [doc = " [`Asn1TimeRef`]: struct.Asn1TimeRef.html"] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct TimeDiff { # [doc = " Difference in days"] pub days : c_int , # [doc = " Difference in seconds."] # [doc = ""] # [doc = " This is always less than the number of seconds in a day."] pub secs : c_int , }
};
}
