// Generated macro for tests (module)
macro_rules! Depcrate_math_rem_pio2tests {
() => {
// Module: crate::math::rem_pio2
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: rem_pio2 ; # [test] # [cfg_attr (x86_no_sse , ignore)] fn test_near_pi () { let arg = 3.141592025756836 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 6.278329573009626e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592033207416 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 6.20382377148128e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592144966125 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 5.086236681942706e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592979431152 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , 3.2584135866119817e-7 , - 2.1125998133974653e-23)) ; } # [test] fn test_overflow_b9b847 () { let _ = rem_pio2 (- 3054214.5490637687) ; } # [test] fn test_overflow_4747b9 () { let _ = rem_pio2 (917340800458.2274) ; } }
};
}
