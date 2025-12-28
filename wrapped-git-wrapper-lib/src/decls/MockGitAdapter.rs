macro_rules! deps {
    () => {
        SubmoduleStat!();
        GitAdapter!();
    };
}

macro_rules! MockGitAdapter {
    () => {
        deps!();
        # [doc = " Mock implementation of `GitAdapter` for dry-run or testing."] pub struct MockGitAdapter { pub mock_submodules : Vec < (String , PathBuf) > , pub mock_submodule_stat : SubmoduleStat , }
    };
}

MockGitAdapter!()