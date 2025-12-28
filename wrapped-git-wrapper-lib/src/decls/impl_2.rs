macro_rules! deps {
    () => {
        DummyGitExecutor!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl DummyGitExecutor { pub fn new () -> Self { DummyGitExecutor } }
    };
}

impl_2!();