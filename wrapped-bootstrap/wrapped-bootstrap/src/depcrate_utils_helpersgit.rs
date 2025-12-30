// Generated macro for git (function)
macro_rules! Depcrate_utils_helpersgit {
() => {
// Module: crate::utils::helpers
// Provides: {"git"}
// Dependencies: {}
# [doc = " Prepares `BootstrapCommand` that runs git inside the source directory if given."] # [doc = ""] # [doc = " Whenever a git invocation is needed, this function should be preferred over"] # [doc = " manually building a git `BootstrapCommand`. This approach allows us to manage"] # [doc = " bootstrap-specific needs/hacks from a single source, rather than applying them on next to every"] # [doc = " git command creation, which is painful to ensure that the required change is applied"] # [doc = " on each one of them correctly."] # [track_caller] pub fn git (source_dir : Option < & Path >) -> BootstrapCommand { let mut git = command ("git") ; git . cached () ; if let Some (source_dir) = source_dir { git . current_dir (source_dir) ; git . env_remove ("GIT_DIR") ; git . env_remove ("GIT_WORK_TREE") . env_remove ("GIT_INDEX_FILE") . env_remove ("GIT_OBJECT_DIRECTORY") . env_remove ("GIT_ALTERNATE_OBJECT_DIRECTORIES") ; } git }
};
}
