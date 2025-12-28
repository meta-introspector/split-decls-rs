macro_rules! deps {
    () => {
        RealRepoStateCollector!();
        GitExecutor!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl RealRepoStateCollector { pub fn new (git_executor : Arc < dyn GitExecutor + Send + Sync > , cargo_metadata_provider : Box < dyn CargoMetadataProvider + Send + Sync > ,) -> Self { RealRepoStateCollector { git_executor , cargo_metadata_provider , } } }
    };
}

impl_65!();