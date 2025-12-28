macro_rules! deps {
    () => {
        GitExecutor!();
    };
}

macro_rules! RealRepoStateCollector {
    () => {
        deps!();
        pub struct RealRepoStateCollector { git_executor : Arc < dyn GitExecutor + Send + Sync > , cargo_metadata_provider : Box < dyn CargoMetadataProvider + Send + Sync > , }
    };
}

RealRepoStateCollector!()