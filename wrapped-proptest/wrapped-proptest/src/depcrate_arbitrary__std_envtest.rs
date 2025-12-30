// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__std_envtest {
() => {
// Module: crate::arbitrary::_std::env
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: num ; use crate :: test_runner :: Config ; no_panic_test ! (args => Args , args_os => ArgsOs , vars => Vars , vars_os => VarsOs , join_paths_error => JoinPathsError , var_error => VarError) ; proptest ! { #! [proptest_config (Config { cases : 65536 , .. Config :: default () })] # [test] fn make_utf16_invalid_doesnt_panic (mut buf in [num :: u16 :: ANY ; 3] , p in 0usize .. 3) { make_utf16_invalid (& mut buf , p) ; } } }
};
}
