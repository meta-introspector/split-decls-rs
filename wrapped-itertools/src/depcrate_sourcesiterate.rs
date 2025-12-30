// Generated macro for Iterate (struct)
macro_rules! Depcrate_sourcesIterate {
() => {
// Module: crate::sources
// Provides: {"Iterate"}
// Dependencies: {}
# [doc = " An iterator that infinitely applies function to value and yields results."] # [doc = ""] # [doc = " This `struct` is created by the [`iterate()`](crate::iterate) function."] # [doc = " See its documentation for more."] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Iterate < St , F > { state : St , f : F , }
};
}
