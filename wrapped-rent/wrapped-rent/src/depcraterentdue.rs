// Generated macro for RentDue (enum)
macro_rules! DepcrateRentDue {
() => {
// Module: crate
// Provides: {"RentDue"}
// Dependencies: {}
# [doc = " The return value of [`Rent::due`]."] # [deprecated (since = "3.1.0" , note = "The concept of rent no longer exists, only rent-exemption")] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum RentDue { # [doc = " Used to indicate the account is rent exempt."] Exempt , # [doc = " The account owes this much rent."] Paying (u64) , }
};
}
