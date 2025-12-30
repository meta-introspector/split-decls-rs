// Generated macro for compile_time_assert_std_error_error (function)
macro_rules! Depcrate_testutilcompile_time_assert_std_error_error {
() => {
// Module: crate::testutil
// Provides: {"compile_time_assert_std_error_error"}
// Dependencies: {}
# [doc = " `compile_time_assert_std_error_error::<T>();` fails to compile if `T`"] # [doc = " doesn't implement `std::error::Error`."] # [cfg (feature = "std")] pub const fn compile_time_assert_std_error_error < T : std :: error :: Error > () { }
};
}
