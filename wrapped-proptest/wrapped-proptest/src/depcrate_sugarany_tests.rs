// Generated macro for any_tests (module)
macro_rules! Depcrate_sugarany_tests {
() => {
// Module: crate::sugar
// Provides: {"any_tests"}
// Dependencies: {}
# [cfg (test)] mod any_tests { proptest ! { # [test] fn test_something (a : bool , b in 25u8 .., c in 25u8 .., _d : () , mut _e : () , ref _f : () , ref mut _g : () , [_ , _] : [() ; 2] ,) { if a { } assert ! (b as usize + c as usize >= 50) ; } } # [test] fn proptest_ext_test () { struct Y (pub u8) ; let _ = proptest_helper ! (@ _EXT _STRAT (_ : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT (x : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT (ref x : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT (mut x : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT (ref mut x : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT ([_ , _] : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT ((& mut & Y (ref x)) : u8)) ; let _ = proptest_helper ! (@ _EXT _STRAT (x in 1 .. 2)) ; let proptest_helper ! (@ _EXT _PAT (_ : u8)) = 1 ; let proptest_helper ! (@ _EXT _PAT (_x : u8)) = 1 ; let proptest_helper ! (@ _EXT _PAT (mut _x : u8)) = 1 ; let proptest_helper ! (@ _EXT _PAT (ref _x : u8)) = 1 ; let proptest_helper ! (@ _EXT _PAT (ref mut _x : u8)) = 1 ; let proptest_helper ! (@ _EXT _PAT ([_ , _] : u8)) = [1 , 2] ; let proptest_helper ! (@ _EXT _PAT ((& mut & Y (ref _x)) : u8)) = & mut & Y (1) ; let proptest_helper ! (@ _EXT _PAT (_x in 1 .. 2)) = 1 ; } }
};
}
