// Generated macro for distance (function)
macro_rules! Depcratedistance {
() => {
// Module: crate
// Provides: {"distance"}
// Dependencies: {}
# [doc = " The distance between two points."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`center()`]"] # [doc = " * [`distance_squared()`]"] # [inline] pub fn distance < T : SimdComplexField , const D : usize > (p1 : & Point < T , D > , p2 : & Point < T , D > ,) -> T :: SimdRealField { (& p2 . coords - & p1 . coords) . norm () }
};
}
