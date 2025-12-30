// Generated macro for core_dir (function)
macro_rules! Depcrate_envcore_dir {
() => {
// Module: crate::env
// Provides: {"core_dir"}
// Dependencies: {}
# [doc = " Return the directory obtained by calling `git --exec-path`."] # [doc = ""] # [doc = " Returns `None` if Git could not be found or if it returned an error."] pub fn core_dir () -> Option < & 'static Path > { GIT_CORE_DIR . as_deref () }
};
}
