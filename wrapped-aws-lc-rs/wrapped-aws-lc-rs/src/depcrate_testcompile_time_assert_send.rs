// Generated macro for compile_time_assert_send (function)
macro_rules! Depcrate_testcompile_time_assert_send {
() => {
// Module: crate::test
// Provides: {"compile_time_assert_send"}
// Dependencies: {}
# [doc = " `compile_time_assert_send::<T>();` fails to compile if `T` doesn't"] # [doc = " implement `Send`."] # [allow (clippy :: extra_unused_type_parameters)] pub fn compile_time_assert_send < T : Send > () { }
};
}
