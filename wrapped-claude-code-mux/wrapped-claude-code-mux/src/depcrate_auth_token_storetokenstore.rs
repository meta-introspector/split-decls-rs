// Generated macro for TokenStore (struct)
macro_rules! Depcrate_auth_token_storeTokenStore {
() => {
// Module: crate::auth::token_store
// Provides: {"TokenStore"}
// Dependencies: {}
# [doc = " Token storage - persists to JSON file"] # [derive (Debug , Clone)] pub struct TokenStore { # [doc = " Path to token storage file"] file_path : PathBuf , # [doc = " In-memory cache of tokens"] tokens : Arc < RwLock < HashMap < String , OAuthToken > > > , }
};
}
