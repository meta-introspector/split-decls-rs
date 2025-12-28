macro_rules! deps {
    () => {
        MockGitAdapter!();
        DummyGitExecutor!();
    };
}

macro_rules! MockGitWrapperLib {
    () => {
        deps!();
        pub struct MockGitWrapperLib { git_executor_impl : DummyGitExecutor , git_repo_operations_impl : MockGitRepositoryOperations , gh_executor_impl : MockGhExecutor , execv_impl : MockExecv , git_adapter_impl : MockGitAdapter , }
    };
}

MockGitWrapperLib!();