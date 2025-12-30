// Generated macro for small_add_from (function)
macro_rules! Depcrate_bigintsmall_add_from {
() => {
// Module: crate::bigint
// Provides: {"small_add_from"}
// Dependencies: {}
# [doc = " Add small integer to bigint starting from offset."] # [inline] pub fn small_add_from (x : & mut VecType , y : Limb , start : usize) -> Option < () > { let mut index = start ; let mut carry = y ; while carry != 0 && index < x . len () { let result = scalar_add (x [index] , carry) ; x [index] = result . 0 ; carry = result . 1 as Limb ; index += 1 ; } if carry != 0 { x . try_push (carry) ? ; } Some (()) }
};
}
