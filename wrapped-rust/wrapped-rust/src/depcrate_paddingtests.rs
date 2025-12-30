// Generated macro for tests (module)
macro_rules! Depcrate_paddingtests {
() => {
// Module: crate::padding
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: constant_time_lt ; # [test] fn test_constant_time_lt () { for a in 0 ..= 255 { for b in 0 ..= 255 { let expected = if a < b { 0xff } else { 0 } ; assert_eq ! (constant_time_lt (a , b) , expected) ; } } } }
};
}
