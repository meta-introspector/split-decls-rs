macro_rules! LibGitAdapter {
    () => {
        # [cfg (feature = "git2_enabled")] pub struct LibGitAdapter ;
    };
}

LibGitAdapter!();