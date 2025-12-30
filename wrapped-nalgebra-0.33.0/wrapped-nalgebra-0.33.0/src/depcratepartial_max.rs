// Generated macro for partial_max (function)
macro_rules! Depcratepartial_max {
() => {
// Module: crate
// Provides: {"partial_max"}
// Dependencies: {}
# [doc = " Return the maximum of `a` and `b` if they are comparable."] # [inline] pub fn partial_max < 'a , T : PartialOrd > (a : & 'a T , b : & 'a T) -> Option < & 'a T > { if let Some (ord) = a . partial_cmp (b) { match ord { Ordering :: Less => Some (b) , _ => Some (a) , } } else { None } }
};
}
