// Generated macro for impl_11 (impl)
macro_rules! Depcrate_auth_oauthimpl_11 {
() => {
// Module: crate::auth::oauth
// Provides: {"impl_11"}
// Dependencies: {}
impl PKCEVerifier { # [doc = " Generate a new PKCE code verifier and challenge"] pub fn generate () -> Self { let mut rng = rand :: thread_rng () ; let random_bytes : Vec < u8 > = (0 .. 32) . map (| _ | rng . gen ()) . collect () ; let verifier = URL_SAFE_NO_PAD . encode (& random_bytes) ; let mut hasher = Sha256 :: new () ; hasher . update (verifier . as_bytes ()) ; let challenge_bytes = hasher . finalize () ; let challenge = URL_SAFE_NO_PAD . encode (& challenge_bytes) ; Self { verifier , challenge } } }
};
}
