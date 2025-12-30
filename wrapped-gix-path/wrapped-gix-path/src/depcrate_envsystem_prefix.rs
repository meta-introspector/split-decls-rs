// Generated macro for system_prefix (function)
macro_rules! Depcrate_envsystem_prefix {
() => {
// Module: crate::env
// Provides: {"system_prefix"}
// Dependencies: {}
# [doc = " Returns the platform dependent system prefix or `None` if it cannot be found (right now only on Windows)."] # [doc = ""] # [doc = " ### Performance"] # [doc = ""] # [doc = " On Windows, the slowest part is the launch of the Git executable in the PATH. This is often"] # [doc = " avoided by inspecting the environment, when launched from inside a Git Bash MSYS2 shell."] # [doc = ""] # [doc = " ### When `None` is returned"] # [doc = ""] # [doc = " This happens only Windows if the git binary can't be found at all for obtaining its executable"] # [doc = " path, or if the git binary wasn't built with a well-known directory structure or environment."] pub fn system_prefix () -> Option < & 'static Path > { if cfg ! (windows) { static PREFIX : LazyLock < Option < PathBuf > > = LazyLock :: new (| | { system_prefix_from_exepath_var (| key | std :: env :: var_os (key)) . or_else (| | system_prefix_from_core_dir (core_dir)) }) ; PREFIX . as_deref () } else { Path :: new ("/") . into () } }
};
}
