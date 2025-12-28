macro_rules! deps {
    () => {
        GitAdapter!();
        GitRepositoryOperations!();
        Execv!();
        GitWrapperLibTrait!();
        MockGitWrapperLib!();
        GitExecutor!();
        GhExecutor!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl GitWrapperLibTrait for MockGitWrapperLib { fn git_executor (& self) -> & dyn GitExecutor { & self . git_executor_impl } fn git_repo_operations (& self) -> & dyn GitRepositoryOperations { & self . git_repo_operations_impl } fn gh_executor (& self) -> & dyn GhExecutor { & self . gh_executor_impl } fn execv_executor (& self) -> & dyn Execv { & self . execv_impl } fn git_adapter (& self) -> & dyn GitAdapter { & self . git_adapter_impl } }
    };
}

impl_98!()