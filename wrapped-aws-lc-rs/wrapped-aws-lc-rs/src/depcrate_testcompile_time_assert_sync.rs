// Generated macro for compile_time_assert_sync (function)
macro_rules! Depcrate_testcompile_time_assert_sync {
() => {
// Module: crate::test
// Provides: {"compile_time_assert_sync"}
// Dependencies: {}
# [doc = " `compile_time_assert_sync::<T>();` fails to compile if `T` doesn't"] # [doc = " implement `Sync`."] # [allow (clippy :: extra_unused_type_parameters)] pub fn compile_time_assert_sync < T : Sync > () { }
};
}
