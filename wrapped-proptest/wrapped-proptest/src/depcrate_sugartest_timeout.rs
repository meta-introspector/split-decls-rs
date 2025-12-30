// Generated macro for test_timeout (module)
macro_rules! Depcrate_sugartest_timeout {
() => {
// Module: crate::sugar
// Provides: {"test_timeout"}
// Dependencies: {}
# [cfg (all (test , feature = "timeout"))] mod test_timeout { proptest ! { #! [proptest_config (crate :: test_runner :: Config { fork : true , .. crate :: test_runner :: Config :: default () })] # [test] fn test_name_set_correctly_for_fork (_ in 0u32 .. 1u32) { } } }
};
}
