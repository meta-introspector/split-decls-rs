// Generated macro for load_github_ctx (function)
macro_rules! Depcrateload_github_ctx {
() => {
// Module: crate
// Provides: {"load_github_ctx"}
// Dependencies: {}
fn load_github_ctx () -> anyhow :: Result < GitHubContext > { let event_name = load_env_var ("GITHUB_EVENT_NAME") ? ; let commit_message = if event_name == "push" { Some (load_env_var ("COMMIT_MESSAGE") ?) } else { None } ; Ok (GitHubContext { event_name , branch_ref : load_env_var ("GITHUB_REF") ? , commit_message }) }
};
}
