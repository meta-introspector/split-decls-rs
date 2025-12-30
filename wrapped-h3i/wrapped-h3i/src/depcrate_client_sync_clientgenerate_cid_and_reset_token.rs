// Generated macro for generate_cid_and_reset_token (function)
macro_rules! Depcrate_client_sync_clientgenerate_cid_and_reset_token {
() => {
// Module: crate::client::sync_client
// Provides: {"generate_cid_and_reset_token"}
// Dependencies: {}
# [doc = " Generate a new pair of Source Connection ID and reset token."] pub fn generate_cid_and_reset_token () -> (quiche :: ConnectionId < 'static > , u128) { let rng = SystemRandom :: new () ; let mut scid = [0 ; quiche :: MAX_CONN_ID_LEN] ; rng . fill (& mut scid [..]) . unwrap () ; let scid = scid . to_vec () . into () ; let mut reset_token = [0 ; 16] ; rng . fill (& mut reset_token [..]) . unwrap () ; let reset_token = u128 :: from_be_bytes (reset_token) ; (scid , reset_token) }
};
}
