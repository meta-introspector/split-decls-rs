// Generated macro for macro_1113 (macro)
macro_rules! Depcrate_cfg_not_testmacro_1113 {
() => {
// Module: crate::cfg_not_test
// Provides: {"macro_1113"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `cfg` that excludes code from `test` builds. (i.e., `#[cfg(not(test))]`)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This may give the false impression that a codebase has 100% coverage, yet actually has untested code."] # [doc = " Enabling this also guards against excessive mockery as well, which is an anti-pattern."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " # fn important_check() {}"] # [doc = " #[cfg(not(test))]"] # [doc = " important_check(); // I'm not actually tested, but not including me will falsely increase coverage!"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " # fn important_check() {}"] # [doc = " important_check();"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub CFG_NOT_TEST , restriction , "enforce against excluding code from test builds" }
};
}
