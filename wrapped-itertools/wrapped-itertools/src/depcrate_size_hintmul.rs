// Generated macro for mul (function)
macro_rules! Depcrate_size_hintmul {
() => {
// Module: crate::size_hint
// Provides: {"mul"}
// Dependencies: {}
# [doc = " Multiply `SizeHint` correctly"] # [inline] pub fn mul (a : SizeHint , b : SizeHint) -> SizeHint { let low = a . 0 . saturating_mul (b . 0) ; let hi = match (a . 1 , b . 1) { (Some (x) , Some (y)) => x . checked_mul (y) , (Some (0) , None) | (None , Some (0)) => Some (0) , _ => None , } ; (low , hi) }
};
}
