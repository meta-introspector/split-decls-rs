// Generated macro for AppState (struct)
macro_rules! Depcrate_server_stateAppState {
() => {
// Module: crate::server::state
// Provides: {"AppState"}
// Dependencies: {}
# [doc = " Application state shared across handlers"] # [derive (Clone)] pub struct AppState { pub config : Arc < tokio :: sync :: RwLock < AppConfig > > , pub router : Router , pub provider_registry : Arc < ProviderRegistry > , pub token_store : TokenStore , pub config_path : PathBuf , pub log_state : LogState , }
};
}
