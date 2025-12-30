// Generated macro for AuthorizationUrl (struct)
macro_rules! Depcrate_auth_oauthAuthorizationUrl {
() => {
// Module: crate::auth::oauth
// Provides: {"AuthorizationUrl"}
// Dependencies: {}
# [doc = " Authorization URL with PKCE"] # [derive (Debug , Clone)] pub struct AuthorizationUrl { pub url : String , pub verifier : PKCEVerifier , }
};
}
