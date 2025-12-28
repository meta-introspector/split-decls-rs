macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Mode { fn to_worktree (self) -> bool { matches ! (self , Mode :: ToGitUnlessBinaryToTextIsPresent | Mode :: ToWorktreeAndBinaryToText) } fn to_git (self) -> bool { matches ! (self , Mode :: ToGitUnlessBinaryToTextIsPresent | Mode :: ToGit) } }
    };
}

impl_99!()