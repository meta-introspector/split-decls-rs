// Generated macro for compile_time_assert_copy (function)
macro_rules! Depcrate_testutilcompile_time_assert_copy {
() => {
// Module: crate::testutil
// Provides: {"compile_time_assert_copy"}
// Dependencies: {}
# [doc = " `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't"] # [doc = " implement `Copy`."] pub const fn compile_time_assert_copy < T : Copy > () { }
};
}
