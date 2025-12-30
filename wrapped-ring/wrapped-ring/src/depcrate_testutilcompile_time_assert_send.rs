// Generated macro for compile_time_assert_send (function)
macro_rules! Depcrate_testutilcompile_time_assert_send {
() => {
// Module: crate::testutil
// Provides: {"compile_time_assert_send"}
// Dependencies: {}
# [doc = " `compile_time_assert_send::<T>();` fails to compile if `T` doesn't"] # [doc = " implement `Send`."] pub const fn compile_time_assert_send < T : Send > () { }
};
}
