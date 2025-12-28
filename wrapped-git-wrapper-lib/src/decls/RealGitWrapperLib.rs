macro_rules! deps {
    () => {
        RealExecv!();
        ShellGitAdapter!();
        SystemGitExecutor!();
        SystemGhExecutor!();
        RealGitRepositoryOperations!();
    };
}

macro_rules! RealGitWrapperLib {
    () => {
        deps!();
        pub struct RealGitWrapperLib { git_executor_impl : SystemGitExecutor , git_repo_operations_impl : RealGitRepositoryOperations , gh_executor_impl : SystemGhExecutor , execv_impl : RealExecv , git_adapter_impl : ShellGitAdapter , }
    };
}

RealGitWrapperLib!()