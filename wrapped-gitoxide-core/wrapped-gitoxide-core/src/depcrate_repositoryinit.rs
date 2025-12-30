// Generated macro for init (function)
macro_rules! Depcrate_repositoryinit {
() => {
// Module: crate::repository
// Provides: {"init"}
// Dependencies: {}
pub fn init (directory : Option < PathBuf >) -> Result < gix :: discover :: repository :: Path > { gix :: create :: into (directory . unwrap_or_default () , gix :: create :: Kind :: WithWorktree , gix :: create :: Options :: default () ,) . with_context (| | "Repository initialization failed") }
};
}
