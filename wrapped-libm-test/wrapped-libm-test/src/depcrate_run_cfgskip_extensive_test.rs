// Generated macro for skip_extensive_test (function)
macro_rules! Depcrate_run_cfgskip_extensive_test {
() => {
// Module: crate::run_cfg
// Provides: {"skip_extensive_test"}
// Dependencies: {}
# [doc = " Check whether extensive actions should be run or skipped."] pub fn skip_extensive_test (ctx : & CheckCtx) -> bool { let t_env = TestEnv :: from_env (ctx) ; ! t_env . should_run_extensive }
};
}
