// Generated macro for ci_gcc_root (function)
macro_rules! Depcrate_core_build_steps_gccci_gcc_root {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"ci_gcc_root"}
// Dependencies: {}
# [doc = " The absolute path to the downloaded GCC artifacts."] # [cfg (not (test))] fn ci_gcc_root (config : & crate :: Config , target : TargetSelection) -> PathBuf { config . out . join (target) . join ("ci-gcc") }
};
}
