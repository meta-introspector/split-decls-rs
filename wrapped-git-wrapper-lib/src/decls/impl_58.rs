macro_rules! deps {
    () => {
        RealGitRepositoryOperations!();
        GitExecutor!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl RealGitRepositoryOperations { pub fn new (git_executor : Arc < dyn GitExecutor + Send + Sync >) -> Self { RealGitRepositoryOperations { git_executor } } }
    };
}

impl_58!();