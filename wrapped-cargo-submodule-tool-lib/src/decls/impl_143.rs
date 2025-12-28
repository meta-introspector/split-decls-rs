macro_rules! deps {
    () => {
        PureRustRepoDiscoverer!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl PureRustRepoDiscoverer { pub fn new (target_org : String , target_branch : String) -> Self { PureRustRepoDiscoverer { target_org , target_branch , } } }
    };
}

impl_143!()