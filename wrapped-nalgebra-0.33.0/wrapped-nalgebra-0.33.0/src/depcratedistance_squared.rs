// Generated macro for distance_squared (function)
macro_rules! Depcratedistance_squared {
() => {
// Module: crate
// Provides: {"distance_squared"}
// Dependencies: {}
# [doc = " The squared distance between two points."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`center()`]"] # [doc = " * [`distance()`]"] # [inline] pub fn distance_squared < T : SimdComplexField , const D : usize > (p1 : & Point < T , D > , p2 : & Point < T , D > ,) -> T :: SimdRealField { (& p2 . coords - & p1 . coords) . norm_squared () }
};
}
