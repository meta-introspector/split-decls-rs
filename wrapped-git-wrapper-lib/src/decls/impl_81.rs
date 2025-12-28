macro_rules! deps {
    () => {
        Execv!();
        SystemGhExecutor!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl SystemGhExecutor { pub fn new (gh_executable_path : PathBuf , executor : Arc < dyn Execv + Send + Sync >) -> Self { SystemGhExecutor { gh_executable_path , executor , } } }
    };
}

impl_81!();