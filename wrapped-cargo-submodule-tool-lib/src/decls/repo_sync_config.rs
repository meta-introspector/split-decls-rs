macro_rules! deps {
    () => {
        RepoSyncConfig!();
    };
}

macro_rules! repo_sync_config {
    () => {
        deps!();
        # [cfg (not (feature = "cargo_repo_sync_lib_enabled"))] pub mod repo_sync_config { use anyhow :: Result ; use std :: path :: Path ; pub struct RepoSyncConfig ; impl RepoSyncConfig { pub fn load_from_file (_path : & Path) -> Result < Self > { unimplemented ! ("RepoSyncConfig is not available without `cargo_repo_sync_lib_enabled` feature.") } } }
    };
}

repo_sync_config!();