// Generated macro for from_float (macro)
macro_rules! Depcrate_testfrom_float {
() => {
// Module: crate::test
// Provides: {"from_float"}
// Dependencies: {}
macro_rules ! from_float { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { $ (mod $ src { mod inf { mod to { use crate :: { Error , From } ; $ (# [test] fn $ dst () { let _0 : $ src = 0. ; let _1 : $ src = 1. ; let inf = _1 / _0 ; let neg_inf = - _1 / _0 ; assert_eq ! ($ dst :: cast (inf) , Err (Error :: Infinite)) ; assert_eq ! ($ dst :: cast (neg_inf) , Err (Error :: Infinite)) ; }) + } } mod nan { mod to { use crate :: { Error , From } ; $ (# [test] fn $ dst () { let _0 : $ src = 0. ; let nan = _0 / _0 ; assert_eq ! ($ dst :: cast (nan) , Err (Error :: NaN)) ; }) + } } }) + } }
};
}
