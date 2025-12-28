macro_rules! deps {
    () => {
        RepoSyncConfig!();
    };
}

macro_rules! run_submodule_status {
    () => {
        deps!();
        # [cfg (not (feature = "cargo_repo_sync_lib_enabled"))] pub mod run_submodule_status { use anyhow :: Result ; use git_wrapper_lib :: git_traits :: GitExecutor ; use git_wrapper_lib :: git_types :: RollupLock ; use super :: repo_sync_config :: RepoSyncConfig ; use std :: collections :: HashMap ; use std :: path :: Path ; use std :: sync :: { Arc , Mutex } ; pub fn run_submodule_status (_git_executor : Arc < dyn GitExecutor + Send + Sync > , _repo_sync_config : & RepoSyncConfig , _project_root : & Path , _rollup_lock : Arc < Mutex < RollupLock > > ,) -> Result < HashMap < String , String > > { unimplemented ! ("run_submodule_status is not available without `cargo_repo_sync_lib_enabled` feature.") } }
    };
}

run_submodule_status!()