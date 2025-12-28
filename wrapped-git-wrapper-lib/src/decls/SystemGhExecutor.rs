macro_rules! deps {
    () => {
        Execv!();
    };
}

macro_rules! SystemGhExecutor {
    () => {
        deps!();
        pub struct SystemGhExecutor { gh_executable_path : PathBuf , executor : Arc < dyn Execv + Send + Sync > , }
    };
}

SystemGhExecutor!();