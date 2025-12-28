macro_rules! deps {
    () => {
        RealGitWrapperLib!();
        RollupLock!();
        RealExecv!();
        RealGitRepositoryOperations!();
        Execv!();
        SystemGitExecutor!();
        GitExecutor!();
        SystemGhExecutor!();
        ShellGitAdapter!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl RealGitWrapperLib { pub fn new () -> Self { let execv_arc : Arc < dyn Execv + Send + Sync > = Arc :: new (RealExecv) ; let git_executable_path = std :: path :: PathBuf :: from ("git") ; let rollup_lock_arc : Arc < Mutex < RollupLock > > = Arc :: new (Mutex :: new (RollupLock :: new ())) ; let root_dir = std :: path :: PathBuf :: from ("/") ; let git_executor_arc : Arc < dyn GitExecutor + Send + Sync > = Arc :: new (SystemGitExecutor :: new (git_executable_path . clone () , execv_arc . clone () , rollup_lock_arc . clone () , root_dir . clone () ,)) ; RealGitWrapperLib { git_executor_impl : SystemGitExecutor :: new (git_executable_path . clone () , execv_arc . clone () , rollup_lock_arc . clone () , root_dir . clone () ,) , git_repo_operations_impl : RealGitRepositoryOperations :: new (git_executor_arc . clone ()) , gh_executor_impl : SystemGhExecutor :: new (std :: path :: PathBuf :: from ("gh") , execv_arc . clone () ,) , execv_impl : RealExecv , git_adapter_impl : ShellGitAdapter :: new (execv_arc . clone ()) , } } }
    };
}

impl_101!();