// Generated macro for impl_27 (impl)
macro_rules! Depcrate_auth_token_storeimpl_27 {
() => {
// Module: crate::auth::token_store
// Provides: {"impl_27"}
// Dependencies: {}
impl OAuthToken { # [doc = " Check if token is expired"] pub fn is_expired (& self) -> bool { Utc :: now () >= self . expires_at } # [doc = " Check if token will expire soon (within 5 minutes)"] pub fn needs_refresh (& self) -> bool { let now = Utc :: now () ; let buffer = chrono :: Duration :: minutes (5) ; now + buffer >= self . expires_at } }
};
}
