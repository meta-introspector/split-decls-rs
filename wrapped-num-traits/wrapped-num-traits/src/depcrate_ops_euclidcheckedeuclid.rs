// Generated macro for CheckedEuclid (trait)
macro_rules! Depcrate_ops_euclidCheckedEuclid {
() => {
// Module: crate::ops::euclid
// Provides: {"CheckedEuclid"}
// Dependencies: {}
pub trait CheckedEuclid : Euclid { # [doc = " Performs euclid division, returning `None` on division by zero or if"] # [doc = " overflow occurred."] fn checked_div_euclid (& self , v : & Self) -> Option < Self > ; # [doc = " Finds the euclid remainder of dividing two numbers, returning `None` on"] # [doc = " division by zero or if overflow occurred."] fn checked_rem_euclid (& self , v : & Self) -> Option < Self > ; # [doc = " Returns both the quotient and remainder from checked Euclidean division,"] # [doc = " returning `None` on division by zero or if overflow occurred."] # [doc = ""] # [doc = " By default, it internally calls both `CheckedEuclid::checked_div_euclid` and `CheckedEuclid::checked_rem_euclid`,"] # [doc = " but it can be overridden in order to implement some optimization."] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use num_traits::CheckedEuclid;"] # [doc = " let x = 5u8;"] # [doc = " let y = 3u8;"] # [doc = ""] # [doc = " let div = CheckedEuclid::checked_div_euclid(&x, &y);"] # [doc = " let rem = CheckedEuclid::checked_rem_euclid(&x, &y);"] # [doc = ""] # [doc = " assert_eq!(Some((div.unwrap(), rem.unwrap())), CheckedEuclid::checked_div_rem_euclid(&x, &y));"] # [doc = " ```"] fn checked_div_rem_euclid (& self , v : & Self) -> Option < (Self , Self) > { Some ((self . checked_div_euclid (v) ? , self . checked_rem_euclid (v) ?)) } }
};
}
