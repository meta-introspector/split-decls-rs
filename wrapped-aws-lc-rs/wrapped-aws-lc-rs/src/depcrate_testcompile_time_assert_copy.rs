// Generated macro for compile_time_assert_copy (function)
macro_rules! Depcrate_testcompile_time_assert_copy {
() => {
// Module: crate::test
// Provides: {"compile_time_assert_copy"}
// Dependencies: {}
# [doc = " `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't"] # [doc = " implement `Copy`."] # [allow (clippy :: extra_unused_type_parameters)] pub fn compile_time_assert_copy < T : Copy > () { }
};
}
