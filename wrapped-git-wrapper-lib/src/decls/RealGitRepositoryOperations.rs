macro_rules! deps {
    () => {
        GitExecutor!();
    };
}

macro_rules! RealGitRepositoryOperations {
    () => {
        deps!();
        pub struct RealGitRepositoryOperations { git_executor : Arc < dyn GitExecutor + Send + Sync > , }
    };
}

RealGitRepositoryOperations!();