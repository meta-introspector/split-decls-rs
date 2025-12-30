// Generated macro for partial_min (function)
macro_rules! Depcratepartial_min {
() => {
// Module: crate
// Provides: {"partial_min"}
// Dependencies: {}
# [doc = " Return the minimum of `a` and `b` if they are comparable."] # [inline] pub fn partial_min < 'a , T : PartialOrd > (a : & 'a T , b : & 'a T) -> Option < & 'a T > { if let Some (ord) = a . partial_cmp (b) { match ord { Ordering :: Greater => Some (b) , _ => Some (a) , } } else { None } }
};
}
