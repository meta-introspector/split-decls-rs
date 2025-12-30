// Generated macro for lazy (function)
macro_rules! Depcrate_common_lazylazy {
() => {
// Module: crate::common::lazy
// Provides: {"lazy"}
// Dependencies: {}
pub (crate) fn lazy < F , R > (func : F) -> Lazy < F , R > where F : FnOnce () -> R , R : Future + Unpin , { Lazy { inner : Inner :: Init { func } , } }
};
}
