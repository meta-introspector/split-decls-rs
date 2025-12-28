macro_rules! deps {
    () => {
        DummyGitExecutor!();
        MockGitAdapter!();
        MockGitWrapperLib!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl MockGitWrapperLib { pub fn new () -> Self { MockGitWrapperLib { git_executor_impl : DummyGitExecutor , git_repo_operations_impl : MockGitRepositoryOperations , gh_executor_impl : MockGhExecutor , execv_impl : MockExecv , git_adapter_impl : MockGitAdapter :: new () , } } }
    };
}

impl_97!();