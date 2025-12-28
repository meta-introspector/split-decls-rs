macro_rules! deps {
    () => {
        MockGitAdapter!();
        SubmoduleStat!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl MockGitAdapter { pub fn new () -> Self { MockGitAdapter { mock_submodules : vec ! [] , mock_submodule_stat : SubmoduleStat { head_commit : "mock_head" . to_string () , workdir_hash : "mock_workdir" . to_string () , } , } } }
    };
}

impl_18!();