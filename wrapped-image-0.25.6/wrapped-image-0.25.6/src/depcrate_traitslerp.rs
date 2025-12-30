// Generated macro for Lerp (trait)
macro_rules! Depcrate_traitsLerp {
() => {
// Module: crate::traits
// Provides: {"Lerp"}
// Dependencies: {}
# [doc = " Linear interpolation without involving floating numbers."] pub trait Lerp : Bounded + NumCast { type Ratio : Primitive ; fn lerp (a : Self , b : Self , ratio : Self :: Ratio) -> Self { let a = < Self :: Ratio as NumCast > :: from (a) . unwrap () ; let b = < Self :: Ratio as NumCast > :: from (b) . unwrap () ; let res = a + (b - a) * ratio ; if res > NumCast :: from (Self :: max_value ()) . unwrap () { Self :: max_value () } else if res < NumCast :: from (0) . unwrap () { NumCast :: from (0) . unwrap () } else { NumCast :: from (res) . unwrap () } } }
};
}
