// Generated macro for tests (module)
macro_rules! Depcrate_auth_token_storetests {
() => {
// Module: crate::auth::token_store
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use tempfile :: TempDir ; # [test] fn test_token_store () { let temp_dir = TempDir :: new () . unwrap () ; let token_path = temp_dir . path () . join ("tokens.json") ; let store = TokenStore :: new (token_path) . unwrap () ; let token = OAuthToken { provider_id : "test-provider" . to_string () , access_token : "access-123" . to_string () , refresh_token : "refresh-456" . to_string () , expires_at : Utc :: now () + chrono :: Duration :: hours (1) , enterprise_url : None , project_id : None , } ; store . save (token . clone ()) . unwrap () ; let retrieved = store . get ("test-provider") . unwrap () ; assert_eq ! (retrieved . access_token , "access-123") ; assert_eq ! (retrieved . refresh_token , "refresh-456") ; store . remove ("test-provider") . unwrap () ; assert ! (store . get ("test-provider") . is_none ()) ; } # [test] fn test_token_expiration () { let expired_token = OAuthToken { provider_id : "test" . to_string () , access_token : "token" . to_string () , refresh_token : "refresh" . to_string () , expires_at : Utc :: now () - chrono :: Duration :: hours (1) , enterprise_url : None , project_id : None , } ; assert ! (expired_token . is_expired ()) ; assert ! (expired_token . needs_refresh ()) ; let valid_token = OAuthToken { provider_id : "test" . to_string () , access_token : "token" . to_string () , refresh_token : "refresh" . to_string () , expires_at : Utc :: now () + chrono :: Duration :: hours (1) , enterprise_url : None , project_id : None , } ; assert ! (! valid_token . is_expired ()) ; assert ! (! valid_token . needs_refresh ()) ; } }
};
}
