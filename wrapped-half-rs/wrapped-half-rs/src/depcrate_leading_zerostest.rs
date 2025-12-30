// Generated macro for test (module)
macro_rules! Depcrate_leading_zerostest {
() => {
// Module: crate::leading_zeros
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn leading_zeros_u16_fallback () { for x in [44 , 97 , 304 , 1179 , 23571] { assert_eq ! (super :: leading_zeros_u16_fallback (x) , x . leading_zeros ()) ; } } }
};
}
