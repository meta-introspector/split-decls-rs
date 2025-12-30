// Generated macro for center (function)
macro_rules! Depcratecenter {
() => {
// Module: crate
// Provides: {"center"}
// Dependencies: {}
# [doc = " The center of two points."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`distance()`]"] # [doc = " * [`distance_squared()`]"] # [inline] pub fn center < T : SimdComplexField , const D : usize > (p1 : & Point < T , D > , p2 : & Point < T , D > ,) -> Point < T , D > { ((& p1 . coords + & p2 . coords) * convert :: < _ , T > (0.5)) . into () }
};
}
