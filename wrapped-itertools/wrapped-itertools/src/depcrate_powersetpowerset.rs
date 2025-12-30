// Generated macro for Powerset (struct)
macro_rules! Depcrate_powersetPowerset {
() => {
// Module: crate::powerset
// Provides: {"Powerset"}
// Dependencies: {}
# [doc = " An iterator to iterate through the powerset of the elements from an iterator."] # [doc = ""] # [doc = " See [`.powerset()`](crate::Itertools::powerset) for more"] # [doc = " information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Powerset < I : Iterator > { combs : Combinations < I > , }
};
}
