// Generated macro for Positions (struct)
macro_rules! Depcrate_adaptorsPositions {
() => {
// Module: crate::adaptors
// Provides: {"Positions"}
// Dependencies: {}
# [doc = " An iterator adapter to get the positions of each element that matches a predicate."] # [doc = ""] # [doc = " See [`.positions()`](crate::Itertools::positions) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Positions < I , F > { iter : Enumerate < I > , f : F , }
};
}
