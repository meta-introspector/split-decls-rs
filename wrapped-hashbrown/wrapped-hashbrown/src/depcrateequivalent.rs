// Generated macro for Equivalent (trait)
macro_rules! DepcrateEquivalent {
() => {
// Module: crate
// Provides: {"Equivalent"}
// Dependencies: {}
# [cfg (not (feature = "equivalent"))] # [doc = " Key equivalence trait."] # [doc = ""] # [doc = " This trait defines the function used to compare the input value with the"] # [doc = " map keys (or set values) during a lookup operation such as [`HashMap::get`]"] # [doc = " or [`HashSet::contains`]."] # [doc = " It is provided with a blanket implementation based on the"] # [doc = " [`Borrow`](core::borrow::Borrow) trait."] # [doc = ""] # [doc = " # Correctness"] # [doc = ""] # [doc = " Equivalent values must hash to the same value."] pub trait Equivalent < K : ? Sized > { # [doc = " Checks if this value is equivalent to the given key."] # [doc = ""] # [doc = " Returns `true` if both values are equivalent, and `false` otherwise."] # [doc = ""] # [doc = " # Correctness"] # [doc = ""] # [doc = " When this function returns `true`, both `self` and `key` must hash to"] # [doc = " the same value."] fn equivalent (& self , key : & K) -> bool ; }
};
}
