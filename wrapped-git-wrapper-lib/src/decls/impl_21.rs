macro_rules! deps {
    () => {
        Execv!();
        ShellGitAdapter!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl ShellGitAdapter { pub fn new (execv : Arc < dyn Execv + Send + Sync >) -> Self { ShellGitAdapter { execv } } }
    };
}

impl_21!();