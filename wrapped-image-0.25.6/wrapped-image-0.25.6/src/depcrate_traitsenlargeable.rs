// Generated macro for Enlargeable (trait)
macro_rules! Depcrate_traitsEnlargeable {
() => {
// Module: crate::traits
// Provides: {"Enlargeable"}
// Dependencies: {}
# [doc = " An `Enlargable::Larger` value should be enough to calculate"] # [doc = " the sum (average) of a few hundred or thousand Enlargeable values."] pub trait Enlargeable : Sized + Bounded + NumCast { type Larger : Copy + NumCast + Num + PartialOrd < Self :: Larger > + Clone + Bounded + AddAssign ; fn clamp_from (n : Self :: Larger) -> Self { if n > Self :: max_value () . to_larger () { Self :: max_value () } else if n < Self :: min_value () . to_larger () { Self :: min_value () } else { NumCast :: from (n) . unwrap () } } fn to_larger (self) -> Self :: Larger { NumCast :: from (self) . unwrap () } }
};
}
