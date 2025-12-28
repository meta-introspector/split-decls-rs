macro_rules! Metadata {
    () => {
        # [derive (Deserialize)] pub struct Metadata { pub target_directory : PathBuf , pub workspace_root : PathBuf , }
    };
}

Metadata!();