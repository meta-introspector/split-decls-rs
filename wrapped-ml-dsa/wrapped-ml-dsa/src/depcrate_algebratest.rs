// Generated macro for test (module)
macro_rules! Depcrate_algebratest {
() => {
// Module: crate::algebra
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: integer_division_remainder_used)] mod test { use super :: * ; use crate :: { MlDsa65 , ParameterSet } ; type Mod = < MlDsa65 as ParameterSet > :: TwoGamma2 ; const MOD : u32 = Mod :: U32 ; const MOD_ELEM : Elem = Elem :: new (MOD) ; # [test] fn mod_plus_minus () { for x in 0 .. MOD { let x = Elem :: new (x) ; let x0 = x . mod_plus_minus :: < Mod > () ; let positive_bound = x0 . 0 <= MOD / 2 ; let negative_bound = x0 . 0 > BaseField :: Q - MOD / 2 ; assert ! (positive_bound || negative_bound) ; let xn = x + MOD_ELEM ; let x0n = x0 + MOD_ELEM ; assert_eq ! (xn . 0 % MOD , x0n . 0 % MOD) ; } } # [test] fn decompose () { for x in 0 .. MOD { let x = Elem :: new (x) ; let (x1 , x0) = x . decompose :: < Mod > () ; let positive_bound = x0 . 0 <= MOD / 2 ; let negative_bound = x0 . 0 >= BaseField :: Q - MOD / 2 ; assert ! (positive_bound || negative_bound) ; let xx = (MOD * x1 . 0 + x0 . 0) % BaseField :: Q ; assert_eq ! (xx , x . 0) ; } } }
};
}
