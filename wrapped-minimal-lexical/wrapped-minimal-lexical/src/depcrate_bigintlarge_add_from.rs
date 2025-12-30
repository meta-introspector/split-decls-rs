// Generated macro for large_add_from (function)
macro_rules! Depcrate_bigintlarge_add_from {
() => {
// Module: crate::bigint
// Provides: {"large_add_from"}
// Dependencies: {}
# [doc = " Add bigint to bigint starting from offset."] pub fn large_add_from (x : & mut VecType , y : & [Limb] , start : usize) -> Option < () > { if y . len () > x . len () . saturating_sub (start) { x . try_resize (y . len () + start , 0) ? ; } let mut carry = false ; for (index , & yi) in y . iter () . enumerate () { let xi = x . get_mut (start + index) . unwrap () ; let result = scalar_add (* xi , yi) ; * xi = result . 0 ; let mut tmp = result . 1 ; if carry { let result = scalar_add (* xi , 1) ; * xi = result . 0 ; tmp |= result . 1 ; } carry = tmp ; } if carry { small_add_from (x , 1 , y . len () + start) ? ; } Some (()) }
};
}
