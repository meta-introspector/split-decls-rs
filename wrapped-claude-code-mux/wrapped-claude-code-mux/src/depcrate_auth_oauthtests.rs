// Generated macro for tests (module)
macro_rules! Depcrate_auth_oauthtests {
() => {
// Module: crate::auth::oauth
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_pkce_generation () { let pkce = PKCEVerifier :: generate () ; assert ! (! pkce . verifier . is_empty ()) ; assert ! (! pkce . challenge . is_empty ()) ; assert_ne ! (pkce . verifier , pkce . challenge) ; } # [test] fn test_authorization_url () { let config = OAuthConfig :: anthropic () ; let token_store = TokenStore :: new (std :: env :: temp_dir () . join ("test_tokens.json")) . unwrap () ; let client = OAuthClient :: new (config , token_store) ; let auth_url = client . get_authorization_url () ; assert ! (auth_url . url . contains ("client_id=")) ; assert ! (auth_url . url . contains ("code_challenge=")) ; assert ! (auth_url . url . contains ("code_challenge_method=S256")) ; assert ! (auth_url . url . contains ("scope=")) ; } }
};
}
