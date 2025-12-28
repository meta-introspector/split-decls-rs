macro_rules! deps {
    () => {
        LibGitAdapter!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "git2_enabled")] impl LibGitAdapter { pub fn new () -> Self { LibGitAdapter } }
    };
}

impl_24!()