// Generated macro for tests (module)
macro_rules! Depcrate_non_zerotests {
() => {
// Module: crate::non_zero
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { ConstChoice , I128 , U128 } ; # [test] fn int_abs_sign () { let x = I128 :: from (- 55) . to_nz () . unwrap () ; let (abs , sgn) = x . abs_sign () ; assert_eq ! (abs , U128 :: from (55u32) . to_nz () . unwrap ()) ; assert_eq ! (sgn , ConstChoice :: TRUE) ; } }
};
}
