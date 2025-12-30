// Generated macro for partial_clamp (function)
macro_rules! Depcratepartial_clamp {
() => {
// Module: crate
// Provides: {"partial_clamp"}
// Dependencies: {}
# [doc = " Clamp `value` between `min` and `max`. Returns `None` if `value` is not comparable to"] # [doc = " `min` or `max`."] # [inline] pub fn partial_clamp < 'a , T : PartialOrd > (value : & 'a T , min : & 'a T , max : & 'a T) -> Option < & 'a T > { if let (Some (cmp_min) , Some (cmp_max)) = (value . partial_cmp (min) , value . partial_cmp (max)) { if cmp_min == Ordering :: Less { Some (min) } else if cmp_max == Ordering :: Greater { Some (max) } else { Some (value) } } else { None } }
};
}
