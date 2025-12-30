// Generated macro for tests (module)
macro_rules! Depcrate_ots_utiltests {
() => {
// Module: crate::ots::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: ots :: util :: coefs ; # [test] fn coef_test_w1 () { let s = [0x12 , 0x34] ; let cs = coefs (& s , 1) . collect :: < Vec < _ > > () ; assert_eq ! (cs , vec ! [0 , 0 , 0 , 1 , 0 , 0 , 1 , 0 , 0 , 0 , 1 , 1 , 0 , 1 , 0 , 0]) ; } # [test] fn coef_test_w2 () { let s = [0x12 , 0x34] ; let cs : Vec < u8 > = coefs (& s , 2) . collect :: < Vec < _ > > () ; assert_eq ! (cs , vec ! [0 , 1 , 0 , 2 , 0 , 3 , 1 , 0]) ; } # [test] fn coef_test_w4 () { let s = [0x12 , 0x34] ; let cs : Vec < u8 > = coefs (& s , 4) . collect :: < Vec < _ > > () ; assert_eq ! (cs , vec ! [1 , 2 , 3 , 4]) ; } # [test] fn coef_test_w8 () { let s = [0x12 , 0x34] ; let cs : Vec < u8 > = coefs (& s , 8) . collect :: < Vec < _ > > () ; assert_eq ! (cs , vec ! [0x12 , 0x34]) ; } }
};
}
