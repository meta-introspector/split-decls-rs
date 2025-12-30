// Generated macro for GIT_HIGHEST_SCOPE_CONFIG_PATH (static)
macro_rules! Depcrate_env_gitGIT_HIGHEST_SCOPE_CONFIG_PATH {
() => {
// Module: crate::env::git
// Provides: {"GIT_HIGHEST_SCOPE_CONFIG_PATH"}
// Dependencies: {}
# [doc = " Invoke the git executable to obtain the origin configuration, which is cached and returned."] # [doc = ""] # [doc = " The git executable is the one found in `PATH` or an alternative location."] pub (super) static GIT_HIGHEST_SCOPE_CONFIG_PATH : LazyLock < Option < BString > > = LazyLock :: new (exe_info) ;
};
}
