// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__core_numtest {
() => {
// Module: crate::arbitrary::_core::num
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (parse_float_error => ParseFloatError , parse_int_error => ParseIntError , wrapping => Wrapping < u8 >, saturating => Saturating < u8 >, fp_category => FpCategory) ; # [cfg (feature = "unstable")] no_panic_test ! (try_from_int_error => TryFromIntError) ; }
};
}
