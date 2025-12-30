// Generated macro for min (function)
macro_rules! Depcrate_size_hintmin {
() => {
// Module: crate::size_hint
// Provides: {"min"}
// Dependencies: {}
# [doc = " Return the minimum"] # [inline] pub fn min (a : SizeHint , b : SizeHint) -> SizeHint { let (a_lower , a_upper) = a ; let (b_lower , b_upper) = b ; let lower = cmp :: min (a_lower , b_lower) ; let upper = match (a_upper , b_upper) { (Some (u1) , Some (u2)) => Some (cmp :: min (u1 , u2)) , _ => a_upper . or (b_upper) , } ; (lower , upper) }
};
}
