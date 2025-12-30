// Generated macro for partial_sort2 (function)
macro_rules! Depcratepartial_sort2 {
() => {
// Module: crate
// Provides: {"partial_sort2"}
// Dependencies: {}
# [doc = " Sorts two values in increasing order using a partial ordering."] # [inline] pub fn partial_sort2 < 'a , T : PartialOrd > (a : & 'a T , b : & 'a T) -> Option < (& 'a T , & 'a T) > { if let Some (ord) = a . partial_cmp (b) { match ord { Ordering :: Less => Some ((a , b)) , _ => Some ((b , a)) , } } else { None } }
};
}
