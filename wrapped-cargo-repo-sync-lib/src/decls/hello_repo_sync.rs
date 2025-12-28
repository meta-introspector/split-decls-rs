macro_rules! hello_repo_sync {
    () => {
        pub fn hello_repo_sync () -> String { "Hello from cargo-repo-sync-lib!" . to_string () }
    };
}

hello_repo_sync!();