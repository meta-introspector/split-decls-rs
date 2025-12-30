// Generated macro for BehaviorOnFailure (enum)
macro_rules! Depcrate_utils_execBehaviorOnFailure {
() => {
// Module: crate::utils::exec
// Provides: {"BehaviorOnFailure"}
// Dependencies: {}
# [doc = " What should be done when the command fails."] # [derive (Debug , Copy , Clone)] pub enum BehaviorOnFailure { # [doc = " Immediately stop bootstrap."] Exit , # [doc = " Delay failure until the end of bootstrap invocation."] DelayFail , # [doc = " Ignore the failure, the command can fail in an expected way."] Ignore , }
};
}
