// Generated macro for impl_156 (impl)
macro_rules! Depcrate_blob_pipelineimpl_156 {
() => {
// Module: crate::blob::pipeline
// Provides: {"impl_156"}
// Dependencies: {}
impl Mode { fn to_worktree (self) -> bool { matches ! (self , Mode :: ToGitUnlessBinaryToTextIsPresent | Mode :: ToWorktreeAndBinaryToText) } fn to_git (self) -> bool { matches ! (self , Mode :: ToGitUnlessBinaryToTextIsPresent | Mode :: ToGit) } }
};
}
