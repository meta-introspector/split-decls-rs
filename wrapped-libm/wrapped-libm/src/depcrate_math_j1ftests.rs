// Generated macro for tests (module)
macro_rules! Depcrate_math_j1ftests {
() => {
// Module: crate::math::j1f
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { j1f , y1f } ; # [test] fn test_j1f_2488 () { assert_eq ! (j1f (2.4881766_f32) , 0.49999475_f32) ; } # [test] fn test_y1f_2002 () { let res = y1f (2.0000002_f32) ; if cfg ! (all (target_arch = "x86" , not (target_feature = "sse2"))) && (res == - 0.10703231_f32) { return ; } assert_eq ! (res , - 0.10703229_f32) ; } }
};
}
